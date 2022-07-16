use bevy::prelude::*;
use bevy_renet::{
    renet::{
        ConnectToken, RenetClient, RenetConnectionConfig, RenetServer, ServerConfig, ServerEvent,
        NETCODE_KEY_BYTES,
    },
    run_if_client_conected, RenetClientPlugin, RenetServerPlugin,
};
use bincode;
use renet::RenetError;
use reqwest;
use serde::{Deserialize, Serialize};
use shared_models::{ConnectionData, LobbyResponse, PROTOCOL_ID};
use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        let args: Vec<String> = std::env::args().collect();
        #[cfg(not(target_arch = "wasm32"))]
        let is_host = {
            let exec_type = &args[1];
            match exec_type.as_str() {
                "client" => false,
                "server" => true,
                _ => panic!("Invalid argument, must be \"client\" or \"server\"."),
            }
        };
        #[cfg(target_arch = "wasm32")]
        let is_host = false;

        app.insert_resource(Lobby::default());

        if is_host {
            app.add_plugin(RenetServerPlugin);
            app.insert_resource(create_renet_server());
            app.add_system(server_update_system);
            app.add_system(server_sync_players);
            // app.add_system(move_players_system);
        } else {
            app.add_plugin(RenetClientPlugin);
            // app.insert_resource(new_renet_client());

            // app.insert_resource(PlayerInput::default());
            // app.add_system(player_input);
            // app.add_system(client_send_input.with_run_criteria(run_if_client_conected));
            app.add_system(client_sync_players.with_run_criteria(run_if_client_conected));
        }

        app.add_system(panic_on_error_system);

        app.run();
    }
}

#[derive(Debug, Component)]
struct Player {
    id: u64,
}

#[derive(Debug, Default)]
struct Lobby {
    players: HashMap<u64, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}

pub async fn create_lobby(username: &str, lobby: &str) -> RenetClient {
    let request = ConnectionData {
        username: username.to_string(),
        lobby: lobby.to_string(),
    };
    create_client_from_request("http://localhost:1337/lobbies", request).await
}

pub async fn join_lobby(username: &str, lobby: &str) -> RenetClient {
    let request = ConnectionData {
        username: username.to_string(),
        lobby: lobby.to_string(),
    };
    let url = format!("http://localhost:1337/lobbies/{}", lobby);
    create_client_from_request(&url, request).await
}

pub fn create_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:1337".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, *PRIVATE_KEY);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

async fn create_client_from_request(url: &str, connection_data: ConnectionData) -> RenetClient {
    let request = reqwest::Client::new()
        .put(url)
        .json(&connection_data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    create_client(request)
}

fn create_client(lobby_response: LobbyResponse) -> RenetClient {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let token = read_token(&lobby_response.token);
    let client_id = lobby_response.client_id;

    RenetClient::new(current_time, socket, client_id, token, connection_config).unwrap()
}

fn read_token(mut bytes: &[u8]) -> ConnectToken {
    let token = ConnectToken::read(&mut bytes).unwrap();
    token
}

fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Player {} connected.", id);
                // Spawn player cube
                let player_entity = commands
                    .spawn()
                    // .insert(PlayerInput::default())
                    .insert(Player { id: *id })
                    .id();

                // We could send an InitState with all the players id and positions for the client
                // but this is easier to do.
                for &player_id in lobby.players.keys() {
                    let message =
                        bincode::serialize(&ServerMessages::PlayerConnected { id: player_id })
                            .unwrap();
                    server.send_message(*id, 0, message);
                }

                lobby.players.insert(*id, player_entity);

                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *id }).unwrap();
                server.broadcast_message(0, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Player {} disconnected.", id);
                if let Some(player_entity) = lobby.players.remove(id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *id }).unwrap();
                server.broadcast_message(0, message);
            }
        }
    }

    for client_id in server.clients_id().into_iter() {
        while let Some(_message) = server.receive_message(client_id, 0) {
            /*
            let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_input);
            }
            */
        }
    }
}

fn server_sync_players(mut server: ResMut<RenetServer>, query: Query<(&Transform, &Player)>) {
    let mut players: HashMap<u64, [f32; 3]> = HashMap::new();
    for (transform, player) in query.iter() {
        players.insert(player.id, transform.translation.into());
    }

    let sync_message = bincode::serialize(&players).unwrap();
    server.broadcast_message(1, sync_message);
}

fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
) {
    while let Some(message) = client.receive_message(0) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);
                let player_entity = commands.spawn().id();

                lobby.players.insert(id, player_entity);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
            }
        }
    }

    while let Some(message) = client.receive_message(1) {
        let players: HashMap<u64, [f32; 3]> = bincode::deserialize(&message).unwrap();
        for (player_id, translation) in players.iter() {
            if let Some(player_entity) = lobby.players.get(player_id) {
                let transform = Transform {
                    translation: (*translation).into(),
                    ..Default::default()
                };
                commands.entity(*player_entity).insert(transform);
            }
        }
    }
}

/*
fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();

    client.send_message(0, input_message);
}
*/

// If any error is found we just panic
fn panic_on_error_system(mut renet_error: EventReader<RenetError>) {
    for e in renet_error.iter() {
        panic!("{}", e);
    }
}

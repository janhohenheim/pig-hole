#[macro_use]
extern crate rocket;

use renet::{ConnectToken, RenetConnectionConfig, NETCODE_KEY_BYTES};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::deadpool_redis::{self, redis};
use rocket_db_pools::{Connection, Database};
use serde_redis::RedisDeserialize;
use shared_models::{Lobby, Username};
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
const PROTOCOL_ID: u64 = 7;
#[derive(Database)]
#[database("lobbies")]
struct Lobbies(deadpool_redis::Pool);

#[get("/lobbies")]
async fn list_lobbies(mut db: Connection<Lobbies>) -> Json<Vec<Lobby>> {
    let lobby_names: Vec<String> = db.smembers("matchmaker/lobbies").await.unwrap();

    let mut lobbies = Vec::new();
    for lobby_name in lobby_names {
        let lobby_value: redis::Value = db.hgetall(&lobby_name).await.unwrap();
        let lobby: Lobby = lobby_value.deserialize().unwrap();
        lobbies.push(lobby);
    }
    Json(lobbies)
}

#[get("/lobbies/<lobby>")]
async fn get_lobby(lobby: String, mut db: Connection<Lobbies>) -> Json<Option<Lobby>> {
    let lobby = query_lobby(&lobby, &mut db).await;
    Json(lobby)
}

async fn query_lobby(lobby: &str, db: &mut Connection<Lobbies>) -> Option<Lobby> {
    let lobby = format!("matchmaker/lobby:{}", lobby);
    let lobby_value: redis::Value = db.hgetall(&lobby).await.unwrap();
    lobby_value.deserialize().ok()
}

#[put("/lobbies", format = "json", data = "<lobby>")]
async fn create_lobby(lobby: Json<Lobby>, mut db: Connection<Lobbies>) -> Status {
    if query_lobby(&lobby.name, &mut db).await.is_some() {
        return Status::Conflict;
    }

    let lobby = lobby.0;
    let hash_name = format!("matchmaker/lobby:{}", lobby.name);

    let _: () = db
        .hset_multiple(
            &hash_name,
            &[
                ("name", lobby.name),
                ("host", lobby.host),
                ("playing", lobby.playing.to_string()),
                ("players", lobby.players.to_string()),
            ],
        )
        .await
        .unwrap();

    let _: () = db.sadd("matchmaker/lobbies", &hash_name).await.unwrap();
    Status::Ok
}

#[put("/lobbies", format = "json", data = "<lobby>")]
async fn set_player_count(lobby: Json<Lobby>, mut db: Connection<Lobbies>) -> Status {}

fn generate_token(username: String) -> ConnectToken {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr: SocketAddr = format!("127.0.0.1:1337").parse().unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let username = Username(username);
    ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        300,
        client_id,
        15,
        vec![server_addr],
        Some(&username.to_netcode_user_data()),
        PRIVATE_KEY,
    )
    .unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Lobbies::init()).mount(
        "/",
        routes![list_lobbies, create_lobby, get_lobby, does_player_exist],
    )
}

use renet::ConnectToken;
use renet::NETCODE_KEY_BYTES;
use shared_models::{ConnectionData, PROTOCOL_ID};
use std::net::SocketAddr;
use std::time::SystemTime;
use uuid::Uuid;

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes

pub fn create_lobby(lobby: &str, host: &str) -> Vec<u8> {
    join_lobby(lobby, host)
}

pub fn join_lobby(lobby: &str, player: &str) -> Vec<u8> {
    let connection_data = ConnectionData::try_new(player, lobby).unwrap();
    let token = generate_token(connection_data);
    serialize_connect_token(token)
}

fn serialize_connect_token(token: ConnectToken) -> Vec<u8> {
    let mut bytes = vec![0u8; NETCODE_KEY_BYTES];
    token.write(&mut bytes).unwrap();
    bytes
}

fn generate_token(connection_data: ConnectionData) -> ConnectToken {
    let server_addr: SocketAddr = format!("127.0.0.1:1337").parse().unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    // This probably eliminates uniqueness guarantee, let's see how that goes.
    let client_id = Uuid::new_v4().as_u64_pair().0;
    ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        300,
        client_id,
        15,
        vec![server_addr],
        Some(&connection_data.to_netcode_user_data()),
        PRIVATE_KEY,
    )
    .unwrap()
}

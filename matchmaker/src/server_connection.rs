use renet::NETCODE_KEY_BYTES;
use renet::{ConnectToken, RenetConnectionConfig};
use shared_models::Username;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use uuid::Uuid;

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
const PROTOCOL_ID: u64 = 7;

pub fn create_lobby(lobby: &str, host: &str) -> ConnectToken {
    generate_token(host.to_string())
}

pub fn join_lobby(lobby: &str, player: &str) -> ConnectToken {
    generate_token(player.to_string())
}

fn generate_token(username: String) -> ConnectToken {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr: SocketAddr = format!("127.0.0.1:1337").parse().unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = Uuid.new_v4().to_u64;
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

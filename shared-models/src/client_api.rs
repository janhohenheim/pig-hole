use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct LobbyCreation {
    pub name: String,
    pub host: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct PlayerCountSettings {
    pub count: u8,
    pub secret: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct LobbyResponse {
    pub token: Vec<u8>,
    pub client_id: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Lobby {
    pub name: String,
    pub playing: bool,
    pub player_count: u8,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Lobby {
    pub name: String,
    pub host: String,
    pub playing: bool,
    pub player_count: u8,
}

#![feature(slice_flatten)]

use renet::NETCODE_USER_DATA_BYTES;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Lobby {
    pub name: String,
    pub playing: bool,
    pub player_count: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ConnectionData {
    pub username: String,
    pub lobby: String,
}

const DATA_PARTS: usize = 2;
type DataLen = u8;
/// Guaranteed to actually use less, since we don't account for the length of the header in the header itself
const HEADER_PART_BYTES: usize = NETCODE_USER_DATA_BYTES / (DataLen::MAX as usize + 1);
/// Format: <length of username> <length of lobby name> <username> <lobby name>
const HEADER_BYTES: usize = HEADER_PART_BYTES * DATA_PARTS;
const MAX_DATA_PART_BYTES: usize = (NETCODE_USER_DATA_BYTES - HEADER_BYTES) / DATA_PARTS;
impl ConnectionData {
    pub fn is_valid_data_part(data: &str) -> bool {
        data.len() <= MAX_DATA_PART_BYTES
    }

    pub fn try_new(username: &str, lobby: &str) -> Option<Self> {
        if !Self::is_valid_data_part(username) || !Self::is_valid_data_part(lobby) {
            return None;
        }

        Self {
            username: username.to_string(),
            lobby: lobby.to_string(),
        }
        .into()
    }

    pub fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        let username_bytes = self.username.as_bytes();
        let lobby_bytes = self.lobby.as_bytes();

        user_data[0..HEADER_PART_BYTES]
            .copy_from_slice((username_bytes.len() as DataLen).to_le_bytes().as_slice());

        user_data[HEADER_PART_BYTES..HEADER_PART_BYTES * 2]
            .copy_from_slice((lobby_bytes.len() as DataLen).to_le_bytes().as_slice());

        (&mut user_data[HEADER_BYTES..HEADER_BYTES + MAX_DATA_PART_BYTES])
            .write(username_bytes)
            .unwrap();
        (&mut user_data[HEADER_BYTES + MAX_DATA_PART_BYTES..])
            .write(lobby_bytes)
            .unwrap();
        user_data
    }

    pub fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let mut username = String::new();
        let mut lobby = String::new();
        let username_len =
            DataLen::from_le_bytes(user_data[0..HEADER_PART_BYTES].try_into().unwrap());
        let lobby_len = DataLen::from_le_bytes(
            user_data[HEADER_PART_BYTES..HEADER_BYTES]
                .try_into()
                .unwrap(),
        );
        (&user_data[HEADER_BYTES..HEADER_BYTES + username_len as usize])
            .read_to_string(&mut username)
            .unwrap();
        (&user_data[HEADER_BYTES + MAX_DATA_PART_BYTES
            ..HEADER_BYTES + MAX_DATA_PART_BYTES + lobby_len as usize])
            .read_to_string(&mut lobby)
            .unwrap();
        Self { username, lobby }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_be_created_from_valid_data() {
        let data = ConnectionData::try_new("username", "lobby");
        assert!(data.is_some());
    }

    #[test]
    fn cannot_be_created_from_invalid_data() {
        let username = std::iter::repeat("a").take(300).collect::<String>();
        let data = ConnectionData::try_new(&username, "lobby");
        assert!(data.is_none());
    }

    #[test]
    fn creates_netcode_bytes() {
        let data = get_valid_connection_data();
        let netcode_bytes = data.to_netcode_user_data();
        assert_eq!(netcode_bytes.len(), NETCODE_USER_DATA_BYTES);
    }

    #[test]
    fn turns_own_netcode_back_into_itself() {
        let sent_data = get_valid_connection_data();
        let netcode_bytes = sent_data.to_netcode_user_data();
        let received_data = ConnectionData::from_user_data(&netcode_bytes);
        assert_eq!(sent_data, received_data);
    }

    #[test]
    fn turns_own_netcode_back_into_itself_with_weird_data() {
        let sent_data = get_valid_weird_connection_data();
        let netcode_bytes = sent_data.to_netcode_user_data();
        let received_data = ConnectionData::from_user_data(&netcode_bytes);
        assert_eq!(sent_data, received_data);
    }

    fn get_valid_connection_data() -> ConnectionData {
        ConnectionData::try_new("username", "lobby").unwrap()
    }

    fn get_valid_weird_connection_data() -> ConnectionData {
        ConnectionData::try_new(" 😊🐬💕😘👌  \n", "\t😊🐬💕aa😘👌  \n").unwrap()
    }
}

use bevy::prelude::*;
use naia_bevy_server::{RoomKey, UserKey};
use std::collections::HashMap;

pub struct Global {
    pub main_room_key: RoomKey,
    pub user_to_prediction_map: HashMap<UserKey, Entity>,
}

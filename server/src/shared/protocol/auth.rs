use bevy::prelude::*;

use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::shared::protocol::Protocol"]
pub struct Auth {
    pub username: Property<String>,
    pub password: Property<String>,
}

impl Auth {
    pub fn new(username: &str, password: &str) -> Self {
        Auth::new_complete(username.to_string(), password.to_string())
    }
}

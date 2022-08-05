use bevy::prelude::*;
// use naia_bevy_client::CommandHistory;
use std::default::Default;

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

impl OwnedEntity {
    pub fn new(confirmed_entity: Entity, predicted_entity: Entity) -> Self {
        OwnedEntity {
            confirmed: confirmed_entity,
            predicted: predicted_entity,
        }
    }
}

#[derive(Default)]
pub struct Global {
    pub owned_entity: Option<OwnedEntity>,
    //pub queued_command: Option<KeyCommand>,
    //pub command_history: CommandHistory<KeyCommand>,
}

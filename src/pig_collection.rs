use bevy::prelude::*;

use crate::{player::Player, GameState};

pub struct PigCollectionPlugin;

// This plugin is responsible to control the game audio
impl Plugin for PigCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_pig_collection),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_pig_collection),
        );
    }
}

fn setup_pig_collection(mut commands: Commands, player_query: Query<&Player>) {}

fn update_pig_collection(mut commands: Commands, player_query: Query<&Player>) {}

use bevy::prelude::*;

use crate::{pig_collection::PigCollection, player::Player, turn::Turn, GameState};

pub struct PlayerCreationPlugin;

impl Plugin for PlayerCreationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_players));
    }
}

fn spawn_players(mut commands: Commands) {
    let player_order = (0..1).map(|_| spawn_player(&mut commands)).collect();
    commands.insert_resource(Turn::new(player_order));
}

fn spawn_player(commands: &mut Commands) -> Entity {
    commands
        .spawn()
        .insert(Player::default())
        .insert(Name::new("Player"))
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(0.0, -230.0, 0.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Name::new("Pig collection"))
                .insert(GlobalTransform::default())
                .insert(Transform::from_xyz(-175., 0.0, 0.0))
                .insert(PigCollection {
                    modify_by: 20,
                    ..default()
                });
        })
        .id()
}

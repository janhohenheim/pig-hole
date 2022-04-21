use crate::{loading::BoardAssetCreator, GameState};
use bevy::prelude::*;

pub struct PigCollectionPlugin;

// This plugin is responsible to control the game audio
impl Plugin for PigCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_pig_collection),
        );
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Component)]
pub struct PigCollection {
    pub modify_by: i32,
    pub pigs: Vec<Entity>,
}

fn update_pig_collection(
    mut commands: Commands,
    mut pig_collection_query: Query<(Entity, &mut PigCollection)>,
    board_asset_creator: Res<BoardAssetCreator>,
) {
    for (pig_collection_entity, mut pig_collection) in pig_collection_query.iter_mut() {
        let pig_count = pig_collection.pigs.len();
        let delta = pig_collection.modify_by;
        if delta > 0 {
            commands
                .entity(pig_collection_entity)
                .with_children(|parent| {
                    for i in 0..delta as usize {
                        let pig =
                            spawn_nth_pig(parent, &board_asset_creator, (pig_count + i) as u32);
                        pig_collection.pigs.push(pig);
                    }
                });
        } else if delta < 0 {
            for entity in pig_collection
                .pigs
                .drain((pig_count - delta.abs() as usize)..)
            {
                commands.entity(entity).despawn_recursive();
            }
        }
        pig_collection.modify_by = 0;
    }
}

fn spawn_nth_pig(
    parent: &mut ChildBuilder,
    board_asset_creator: &Res<BoardAssetCreator>,
    n: u32,
) -> Entity {
    parent
        .spawn_bundle(board_asset_creator.get_pig())
        .insert(Transform::from_translation(
            get_relative_position_of_nth_pig(n, board_asset_creator),
        ))
        .insert(Name::new(format!("Pig {} in collection", n + 1)))
        .id()
}

fn get_relative_position_of_nth_pig(n: u32, board_asset_creator: &Res<BoardAssetCreator>) -> Vec3 {
    let padding = board_asset_creator.get_pig_collection_padding();
    let aabb = board_asset_creator.get_pig_aabb();
    let x_index = (n as f32 / 2.0).floor();
    let y_index = (n % 2) as f32;
    Vec3::new(
        x_index * (aabb.x + padding),
        -y_index * (aabb.y + padding),
        3.0,
    )
}

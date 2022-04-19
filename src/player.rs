use crate::actions::Actions;
use crate::board::PigId;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_camera))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(place_pig));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn place_pig(mut pig_visibility_query: Query<(&mut Visibility, &PigId)>, actions: Res<Actions>) {
    if let Some(selected_pig_id) = actions.selected_mould {
        for (mut visibility, pig_id) in pig_visibility_query.iter_mut() {
            if *pig_id == selected_pig_id {
                visibility.is_visible = true;
            }
        }
    }
}

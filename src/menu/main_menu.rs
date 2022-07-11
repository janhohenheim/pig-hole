use crate::loading::FontAssets;
use crate::loading::MenuAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button));
    }
}

fn setup_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    let assets = MenuAssets::new(&font_assets).button;
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(assets.create_button(120.0, 50.0))
        .with_children(|parent| {
            parent.spawn_bundle(assets.create_text("Play".to_string()));
        });
}

#[allow(clippy::type_complexity)]
fn click_play_button(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (button, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            commands.entity(button).despawn_recursive();
            state.set(GameState::JoiningLobby).unwrap();
        }
    }
}

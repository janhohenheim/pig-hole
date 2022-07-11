use crate::loading::FontAssets;
use crate::loading::MenuAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct JoiningLobbyPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for JoiningLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::JoiningLobby).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::JoiningLobby).with_system(click_play_button),
            );
    }
}

fn setup_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    let assets = MenuAssets::new(&font_assets);
    commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::ALICE_BLUE),
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(50.0)),
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Relative,
                position: Rect {
                    left: Val::Percent(40.0),
                    bottom: Val::Percent(15.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(assets.button.create_text("Name:".to_string()));
            parent
                .spawn_bundle(assets.button.create_textbox(120.0, 50.0))
                .with_children(|parent| {
                    parent.spawn_bundle(assets.button.create_subtext("".to_string()));
                });
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
            state.set(GameState::Lobby).unwrap();
        }
    }
}

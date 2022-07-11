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

fn setup_menu(mut commands: Commands, font_assets: Res<FontAssets>, menu_assets: Res<MenuAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: menu_assets.button.colors.normal,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Player Name:".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

#[allow(clippy::type_complexity)]
fn click_play_button(
    mut commands: Commands,
    menu_assets: Res<MenuAssets>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                commands.entity(button).despawn_recursive();
                state.set(GameState::JoiningLobby).unwrap();
            }
            Interaction::Hovered => {
                *color = menu_assets.button.colors.hovered;
            }
            Interaction::None => {
                *color = menu_assets.button.colors.normal;
            }
        }
    }
}

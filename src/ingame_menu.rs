use crate::loading::FontAssets;
use crate::player::{Player, PlayerState};
use crate::GameState;
use bevy::prelude::*;

pub struct IngameMenuPlugin;

impl Plugin for IngameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(click_dice_button),
            );
    }
}

struct ButtonColors {
    normal: UiColor,
    inactive: UiColor,
    hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            inactive: Color::rgba(0.15, 0.15, 0.15, 0.5).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Component)]
struct DiceRollButton;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(600.0),
                    bottom: Val::Px(300.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Ingame menu"))
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                        position_type: PositionType::Relative,
                        border: Rect::all(Val::Px(20.0)),
                        ..default()
                    },
                    color: button_colors.normal,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Roll dice".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..default()
                    });
                })
                .insert(Name::new("Dice roll button"))
                .insert(DiceRollButton);
        });
}

#[allow(clippy::type_complexity)]
fn click_dice_button(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor),
        (Changed<Interaction>, (With<Button>, With<DiceRollButton>)),
    >,
    mut player_query: Query<&mut Player>,
    mut text_query: Query<(&Parent, &mut Text)>,
) {
    for (entity, interaction, mut color) in interaction_query.iter_mut() {
        for mut player in player_query.iter_mut() {
            match *interaction {
                Interaction::Clicked => match player.state {
                    PlayerState::Selecting(_) => (),
                    PlayerState::ThrowingDice() => {
                        let roll = player.throw_dice();
                        for (parent, mut text) in text_query.iter_mut() {
                            if parent.0 == entity {
                                text.sections[0].value = format!("Rolled a {}", roll);
                            }
                        }
                        *color = button_colors.inactive;
                    }
                },
                Interaction::Hovered => match player.state {
                    PlayerState::Selecting(_) => (),
                    PlayerState::ThrowingDice() => *color = button_colors.hovered,
                },
                Interaction::None => match player.state {
                    PlayerState::Selecting(_) => *color = button_colors.inactive,
                    PlayerState::ThrowingDice() => *color = button_colors.normal,
                },
            }
        }
    }
}

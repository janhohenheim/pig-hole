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
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_click_dice_button)
                    .with_system(handle_player_state)
                    .with_system(present_view_model),
            )
            .init_resource::<ViewModel>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct ViewModel {
    pub dice_roll_button_text: String,
    pub is_dice_roll_button_enabled: bool,
    pub is_dice_roll_button_hovered: bool,
    pub info_text_lines: Vec<String>,
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
struct DiceRollNode;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Component)]
struct EndTurnNode;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(180.0), Val::Px(200.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::SpaceBetween,
                flex_direction: FlexDirection::ColumnReverse,
                position: Rect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(40.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("Ingame menu"))
        .with_children(|parent| {
            spawn_button(
                parent,
                &button_colors,
                &font_assets,
                DiceRollNode,
                "Dice roll button",
            );
            spawn_button(
                parent,
                &button_colors,
                &font_assets,
                EndTurnNode,
                "End turn button",
            );
        });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    button_colors: &Res<ButtonColors>,
    font_assets: &Res<FontAssets>,
    tag: impl Component + Clone,
    name: &'static str,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(45.0)),
                border: Rect {
                    left: Val::Percent(12.0),
                    bottom: Val::Percent(15.0),
                    ..default()
                },
                ..default()
            },
            color: button_colors.normal,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            ..default()
                        }],
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    },
                    ..default()
                })
                .insert(tag.clone());
        })
        .insert(Name::new(name))
        .insert(tag);
}

fn present_view_model(
    button_colors: Res<ButtonColors>,
    mut dice_roll_button_text: Query<&mut Text, With<DiceRollNode>>,
    mut dice_roll_button_color_query: Query<&mut UiColor, (With<Button>, With<DiceRollNode>)>,
    view_model: Res<ViewModel>,
) {
    for mut text in dice_roll_button_text.iter_mut() {
        text.sections[0].value = view_model.dice_roll_button_text.clone()
    }
    for mut color in dice_roll_button_color_query.iter_mut() {
        *color = if view_model.is_dice_roll_button_enabled {
            if view_model.is_dice_roll_button_hovered {
                button_colors.hovered
            } else {
                button_colors.normal
            }
        } else {
            button_colors.inactive
        };
    }
}

#[allow(clippy::type_complexity)]
fn handle_click_dice_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, (With<Button>, With<DiceRollNode>)),
    >,
    mut player_query: Query<&mut Player>,
    mut view_model: ResMut<ViewModel>,
) {
    for interaction in interaction_query.iter_mut() {
        for mut player in player_query.iter_mut() {
            match *interaction {
                Interaction::Clicked => match player.state {
                    PlayerState::Thinking() => player.state = PlayerState::ThrowingDice(),
                    _ => (),
                },
                Interaction::Hovered => view_model.is_dice_roll_button_hovered = true,
                Interaction::None => view_model.is_dice_roll_button_hovered = false,
            }
        }
    }
}

fn handle_player_state(
    player_query: Query<&Player, Changed<Player>>,
    mut view_model: ResMut<ViewModel>,
) {
    for player in player_query.iter() {
        match player.state {
            PlayerState::PlacingInGroup(roll) => {
                view_model.dice_roll_button_text = format!("Rolled a {}", roll);
                view_model.is_dice_roll_button_enabled = false;
            }
            PlayerState::CollectingGroup(roll) => {
                view_model.dice_roll_button_text = format!("Rolled a {}\nCollect", roll);
                view_model.is_dice_roll_button_enabled = false;
            }
            PlayerState::ThrowingDice() => (),
            PlayerState::Thinking() => {
                view_model.dice_roll_button_text = format!("Roll dice");
                view_model.is_dice_roll_button_enabled = true;
            }
        }
    }
}

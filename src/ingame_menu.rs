use crate::loading::FontAssets;
use crate::player::{Player, PlayerInteractionModel, PlayerState};
use crate::turn::Turn;
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
                    .with_system(sync_interaction_model)
                    .with_system(update_info_text)
                    .with_system(present_view_model),
            )
            .init_resource::<ViewModel>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ViewModel {
    pub dice_roll_button: ButtonViewModel,
    pub end_turn_button: ButtonViewModel,
    pub info_text_box: TextBoxViewModel,
}

impl Default for ViewModel {
    fn default() -> Self {
        Self {
            dice_roll_button: ButtonViewModel {
                text: "Roll Dice".to_string(),
                ..default()
            },
            end_turn_button: ButtonViewModel {
                text: "End Turn".to_string(),
                ..default()
            },
            info_text_box: default(),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ButtonViewModel {
    pub is_enabled: bool,
    pub is_hovered: bool,
    pub text: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct TextBoxViewModel {
    pub text_lines: Vec<String>,
}

impl Default for ButtonViewModel {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_hovered: false,
            text: Default::default(),
        }
    }
}

impl Default for TextBoxViewModel {
    fn default() -> Self {
        Self {
            text_lines: vec![" ".to_string(); 3],
        }
    }
}
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct InteractionModel<T: Clone = ()> {
    is_allowed: bool,
    request: Option<T>,
}
impl<T: Clone> InteractionModel<T> {
    pub fn interact(&mut self, value: T) {
        if self.is_allowed {
            self.request = Some(value);
        }
    }

    pub fn allow(&mut self) {
        self.is_allowed = true;
    }

    pub fn deny(&mut self) {
        self.is_allowed = false;
        self.request = None;
    }

    pub fn is_allowed(&self) -> bool {
        self.is_allowed
    }

    pub fn get_interaction(&self) -> Option<T> {
        self.request.clone()
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
struct DiceRollNode;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Component)]
struct EndTurnNode;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Component)]
struct InfoNode;

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
                    left: Val::Percent(70.0),
                    bottom: Val::Percent(20.0),
                    right: Val::Percent(5.0),
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

            spawn_text(parent, &font_assets, 3, InfoNode);
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
                size: Size::new(Val::Auto, Val::Percent(35.0)),
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

fn spawn_text(
    parent: &mut ChildBuilder,
    font_assets: &Res<FontAssets>,
    section_count: usize,
    tag: impl Component,
) {
    parent
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(15.0)),
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        ..default()
                    };
                    section_count
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Left,
                },
            },
            ..default()
        })
        .insert(tag);
}

fn sync_interaction_model(
    interaction_model: ResMut<PlayerInteractionModel>,
    mut view_model: ResMut<ViewModel>,
) {
    view_model.dice_roll_button.is_enabled = interaction_model.roll_dice.is_allowed();
    view_model.end_turn_button.is_enabled = interaction_model.end_turn.is_allowed();
}

fn present_view_model(
    button_colors: Res<ButtonColors>,
    mut texts: ParamSet<(
        Query<&mut Text, With<DiceRollNode>>,
        Query<&mut Text, With<EndTurnNode>>,
        Query<&mut Text, With<InfoNode>>,
    )>,
    mut colors: ParamSet<(
        Query<&mut UiColor, (With<Button>, With<DiceRollNode>)>,
        Query<&mut UiColor, (With<Button>, With<EndTurnNode>)>,
    )>,
    view_model: Res<ViewModel>,
) {
    present_button(
        &button_colors,
        texts.p0(),
        colors.p0(),
        &view_model.dice_roll_button,
    );
    present_button(
        &button_colors,
        texts.p1(),
        colors.p1(),
        &view_model.end_turn_button,
    );
    present_text_box(texts.p2(), &view_model.info_text_box);
}

fn present_button(
    button_colors: &Res<ButtonColors>,
    mut text_query: Query<&mut Text, With<impl Component>>,
    mut color_query: Query<&mut UiColor, (With<Button>, With<impl Component>)>,
    view_model: &ButtonViewModel,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = view_model.text.clone()
    }
    for mut color in color_query.iter_mut() {
        *color = if view_model.is_enabled {
            if view_model.is_hovered {
                button_colors.hovered
            } else {
                button_colors.normal
            }
        } else {
            button_colors.inactive
        };
    }
}

fn present_text_box(
    mut text_query: Query<&mut Text, With<impl Component>>,
    view_model: &TextBoxViewModel,
) {
    for mut text in text_query.iter_mut() {
        for (section, line) in text.sections.iter_mut().zip(view_model.text_lines.iter()) {
            section.value = line.clone()
        }
    }
}

#[allow(clippy::type_complexity)]
fn handle_click_dice_button(
    mut interactions: ParamSet<(
        Query<&Interaction, (Changed<Interaction>, (With<Button>, With<DiceRollNode>))>,
        Query<&Interaction, (Changed<Interaction>, (With<Button>, With<EndTurnNode>))>,
    )>,
    mut player_interaction_model: ResMut<PlayerInteractionModel>,
    mut view_model: ResMut<ViewModel>,
) {
    handle_button_interaction(
        interactions.p0(),
        &mut view_model.dice_roll_button,
        &mut player_interaction_model.roll_dice,
    );
    handle_button_interaction(
        interactions.p1(),
        &mut view_model.end_turn_button,
        &mut player_interaction_model.end_turn,
    );
}

fn handle_button_interaction(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, (With<Button>, With<impl Component>)),
    >,
    view_model: &mut ButtonViewModel,
    interaction_model: &mut InteractionModel,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => interaction_model.interact(()),
            Interaction::Hovered => view_model.is_hovered = true,
            Interaction::None => view_model.is_hovered = false,
        }
    }
}

fn update_info_text(
    player_query: Query<&Player>,
    mut view_model: ResMut<ViewModel>,
    turn: Res<Turn>,
) {
    let lines = &mut view_model.info_text_box.text_lines;
    lines[0] = match turn.get_min_actions() {
        Some(min) => match turn.get_turn_number() {
            1 => format!("Turn {}, everyone needs to roll {} time\n", 1, min),
            turn => format!("Turn {}, everyone needs to roll {} times\n", turn, min),
        },
        None => format!("Turn {}\n", turn.get_turn_number()),
    };
    for player in player_query.iter() {
        match player.state {
            PlayerState::PlacingInGroup(group) => {
                lines[1] = get_roll_info_text(group);
                lines[2] = match turn.get_min_actions() {
                    Some(min) => {
                        if player.action_count == min - 1 {
                            if group == 6 {
                                "Place a pig in the pig hole to end your turn\n".to_string()
                            } else {
                                "Place a pig in a trough to end your turn\n".to_string()
                            }
                        } else {
                            if group == 6 {
                                "Place a pig in the pig hole\n".to_string()
                            } else {
                                "Place a pig in a trough\n".to_string()
                            }
                        }
                    }
                    None => {
                        if group == 6 {
                            "Place a pig in the pig hole\n".to_string()
                        } else {
                            "Place a pig in one of the corresponding troughs\n".to_string()
                        }
                    }
                }
            }
            PlayerState::CollectingGroup(group) => {
                lines[1] = get_roll_info_text(group);
                lines[2] = "The troughs are full. Collect the pigs to end your turn\n".to_string();
            }
            PlayerState::Thinking() => {
                lines[1] = match turn.get_min_actions() {
                    Some(min) => {
                        let actions_left = min - player.action_count;
                        match actions_left {
                            1 => "You need to roll 1 more time\n".to_string(),
                            _ => format!("You need to roll {} more times\n", actions_left),
                        }
                    }
                    None => match player.action_count {
                        0 => "You need to roll at least once\n".to_string(),
                        _ => format!("Roll as much as you want, then end your turn\n"),
                    },
                };
                lines[2] = " ".to_string();
            }
            PlayerState::ThrowingDice() => (),
            PlayerState::Waiting() => {
                lines[1] = "Waiting for your turn".to_string();
                lines[2] = " ".to_string();
            }
        }
    }
}

fn get_roll_info_text(roll: u8) -> String {
    if roll == 6 {
        format!("You rolled a {}!\n", roll)
    } else {
        format!("You rolled a {}\n", roll)
    }
}

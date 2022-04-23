use crate::loading::FontAssets;
use crate::player::PlayerInteractionModel;
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
                    .with_system(present_view_model),
            )
            .init_resource::<ViewModel>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ViewModel {
    pub dice_roll_button: ButtonViewModel,
    pub end_turn_button: ButtonViewModel,
    pub info_text_lines: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ButtonViewModel {
    pub is_enabled: bool,
    pub is_hovered: bool,
    pub text: String,
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
            info_text_lines: vec![],
        }
    }
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

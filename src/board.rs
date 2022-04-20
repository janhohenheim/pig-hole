use std::fmt::Display;

use crate::loading::BoardAssetCreator;
use crate::player::Player;
use crate::player::PlayerState;
use crate::GameState;
use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::Inspectable;
#[cfg(feature = "dev")]
use bevy_inspector_egui::RegisterInspectable;
use bevy_prototype_lyon::prelude::*;

#[cfg_attr(feature = "dev", derive(Inspectable))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Component)]
pub enum PigStatus {
    Empty,
    Occupied,
    PlacementGhost,
    RemovalGhost,
}

#[cfg_attr(feature = "dev", derive(Inspectable))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Component)]
pub struct Pig {
    pub trough: Trough,
    pub status: PigStatus,
}

impl Pig {
    pub fn in_trough(trough: Trough) -> Self {
        Self {
            trough,
            status: PigStatus::Empty,
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.status == PigStatus::Occupied || self.status == PigStatus::RemovalGhost
    }
}
#[cfg_attr(feature = "dev", derive(Inspectable))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Component)]
pub struct Trough {
    pub group: u8,
    pub index: u8,
}

impl From<(u8, u8)> for Trough {
    fn from((group, index): (u8, u8)) -> Self {
        Self { group, index }
    }
}

impl Display for Trough {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.group, self.index)
    }
}
#[cfg_attr(feature = "dev", derive(Inspectable))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Component)]
pub struct Highlight {
    active: bool,
}

impl Trough {
    pub fn new(group: u8, index: u8) -> Self {
        if index > group {
            panic!("inner cannot be greater than outer");
        }
        if group > 6 {
            panic!("outer cannot be greater than 6");
        }
        if group == 0 {
            panic!("outer cannot be 0");
        }
        if index == 0 {
            panic!("inner cannot be 0");
        }

        Self { group, index }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_board))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_pig_visibility)
                    .with_system(update_highlight_visibility)
                    .with_system(activate_highlights),
            );
        #[cfg(feature = "dev")]
        {
            app.register_inspectable::<Pig>();
        }
    }
}

fn spawn_board(mut commands: Commands, board_assets: Res<BoardAssetCreator>) {
    let board_size = Vec3::new(
        board_assets.get_board_extents().x,
        board_assets.get_board_extents().y,
        0.0,
    );
    let quadrant_translation = Vec2::new(board_size.x / 2., board_size.y / 2.);
    let padding = board_assets.get_board_padding();
    let quadrant_offset = board_size / 2. + padding;

    let inner_padding = 5.;
    let offset_right = quadrant_translation / 5. + inner_padding;
    let offset_left = Vec2::new(-offset_right.x, offset_right.y);

    commands
        .spawn()
        .insert(Name::new("Board"))
        .insert(GlobalTransform::default())
        .insert(Transform::default())
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Name::new("Border"))
                .insert(GlobalTransform::default())
                .insert(Transform::default())
                .with_children(|parent| {
                    parent
                        .spawn_bundle(board_assets.get_border())
                        .insert(Name::new("Border"));
                });
            create_trough(parent, Trough::new(6, 1), Vec2::new(0., 0.), &board_assets);

            let top_right_offset = quadrant_offset;
            parent
                .spawn()
                .insert(Name::new("Top right"))
                .insert(GlobalTransform::default())
                .insert(Transform::from_translation(top_right_offset))
                .with_children(|parent| {
                    create_trough(parent, Trough::new(1, 1), -2. * offset_right, &board_assets);

                    parent
                        .spawn()
                        .insert(Name::new("Trough 2"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            create_trough(parent, Trough::new(2, 1), Vec2::ZERO, &board_assets);
                            create_trough(
                                parent,
                                Trough::new(2, 2),
                                2. * offset_right,
                                &board_assets,
                            );
                        });
                });

            let top_left_offset = Vec3::new(-quadrant_offset.x, quadrant_offset.y, 0.);
            parent
                .spawn()
                .insert(Name::new("Top left"))
                .insert(GlobalTransform::default())
                .insert(Transform::from_translation(top_left_offset))
                .with_children(|parent| {
                    parent
                        .spawn()
                        .insert(Name::new("Trough 3"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            create_trough(
                                parent,
                                Trough::new(3, 1),
                                2. * offset_left,
                                &board_assets,
                            );

                            create_trough(parent, Trough::new(3, 2), Vec2::ZERO, &board_assets);

                            create_trough(
                                parent,
                                Trough::new(3, 3),
                                -2. * offset_left,
                                &board_assets,
                            );
                        });
                });

            let bottom_left_offset = Vec3::new(-quadrant_offset.x, -quadrant_offset.y, 0.);
            parent
                .spawn()
                .insert(Name::new("Bottom left"))
                .insert(GlobalTransform::default())
                .insert(Transform::from_translation(bottom_left_offset))
                .with_children(|parent| {
                    parent
                        .spawn()
                        .insert(Name::new("Trough 4"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            create_trough(
                                parent,
                                Trough::new(4, 1),
                                2. * offset_right,
                                &board_assets,
                            );

                            create_trough(
                                parent,
                                Trough::new(4, 2),
                                2. * offset_left,
                                &board_assets,
                            );

                            create_trough(
                                parent,
                                Trough::new(4, 3),
                                -2. * offset_right,
                                &board_assets,
                            );

                            create_trough(
                                parent,
                                Trough::new(4, 4),
                                -2. * offset_left,
                                &board_assets,
                            );
                        });
                });

            let bottom_right_offset = Vec3::new(quadrant_offset.x, -quadrant_offset.y, 0.);
            parent
                .spawn()
                .insert(Name::new("Bottom right"))
                .insert(GlobalTransform::default())
                .insert(Transform::from_translation(bottom_right_offset))
                .with_children(|parent| {
                    parent
                        .spawn()
                        .insert(Name::new("Trough 5"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            create_trough(
                                parent,
                                Trough::new(5, 1),
                                2. * offset_right,
                                &board_assets,
                            );

                            create_trough(
                                parent,
                                Trough::new(5, 2),
                                2. * offset_left,
                                &board_assets,
                            );

                            create_trough(
                                parent,
                                Trough::new(5, 3),
                                -2. * offset_right,
                                &board_assets,
                            );

                            create_trough(
                                parent,
                                Trough::new(5, 4),
                                -2. * offset_left,
                                &board_assets,
                            );

                            create_trough(parent, Trough::new(5, 5), Vec2::ZERO, &board_assets);
                        });
                });
        });
}

fn create_trough(
    parent: &mut ChildBuilder,
    trough: Trough,
    position: Vec2,
    board_assets: &Res<BoardAssetCreator>,
) {
    parent
        .spawn_bundle(board_assets.get_trough_for_group(trough.group))
        .insert(Transform::from_translation(Vec3::new(
            position.x, position.y, 1.,
        )))
        .insert(Name::new(format!("Trough {}", trough)))
        .with_children(|parent| {
            let pig = Pig::in_trough(trough);
            parent
                .spawn_bundle(board_assets.get_pig())
                .insert(pig)
                .insert(Name::new(format!("Pig {}", pig.trough)))
                .insert(Visibility { is_visible: false })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(board_assets.get_highlight())
                        .insert(Name::new(format!("Highlight {}", pig.trough)))
                        .insert(Visibility { is_visible: false })
                        .insert(Highlight { active: false });
                });
        });
}

fn update_pig_visibility(mut pig_query: Query<(&mut Pig, &mut DrawMode, &mut Visibility)>) {
    for (mut pig, mut draw_mode, mut visibility) in pig_query.iter_mut() {
        if pig.trough.group == 6 && pig.is_occupied() {
            pig.status = PigStatus::Empty;
        }
        match pig.status {
            PigStatus::Empty => visibility.is_visible = false,
            PigStatus::Occupied => {
                visibility.is_visible = true;
                *draw_mode = with_alpha(&draw_mode, 1.0);
            }
            PigStatus::PlacementGhost => {
                visibility.is_visible = true;
                *draw_mode = with_alpha(&draw_mode, 0.5);
            }
            PigStatus::RemovalGhost => {
                *draw_mode = with_alpha(&draw_mode, 0.6);
            }
        }
    }
}

fn with_alpha(draw_mode: &DrawMode, alpha: f32) -> DrawMode {
    match *draw_mode {
        DrawMode::Fill(mut fill_mode) => {
            fill_mode.color.set_a(alpha);
            DrawMode::Fill(fill_mode)
        }
        DrawMode::Stroke(mut stroke_mode) => {
            stroke_mode.color.set_a(alpha);
            DrawMode::Stroke(stroke_mode)
        }
        DrawMode::Outlined {
            mut fill_mode,
            mut outline_mode,
        } => {
            fill_mode.color.set_a(alpha);
            outline_mode.color.set_a(alpha);
            DrawMode::Outlined {
                fill_mode,
                outline_mode,
            }
        }
    }
}

fn update_highlight_visibility(mut highlight_query: Query<(&Highlight, &mut Visibility)>) {
    for (highlight, mut visibility) in highlight_query.iter_mut() {
        visibility.is_visible = highlight.active;
    }
}

fn activate_highlights(
    pig_query: Query<(Entity, &Pig)>,
    mut highlight_query: Query<(&Parent, &mut Highlight)>,
    player_query: Query<&Player>,
) {
    for player in player_query.iter() {
        match player.state {
            PlayerState::PlacingInGroup(group) => {
                for (pig_entity, &pig) in pig_query.iter() {
                    if pig.trough.group == group && !pig.is_occupied() {
                        for (parent, mut highlight) in highlight_query.iter_mut() {
                            if parent.0 == pig_entity {
                                highlight.active = true;
                            }
                        }
                    }
                }
            }
            PlayerState::CollectingGroup(group) => {
                for (pig_entity, &pig) in pig_query.iter() {
                    if pig.trough.group == group {
                        for (parent, mut highlight) in highlight_query.iter_mut() {
                            if parent.0 == pig_entity {
                                highlight.active = true;
                            }
                        }
                    }
                }
            }
            PlayerState::Thinking() => {
                for (_parent, mut highlight) in highlight_query.iter_mut() {
                    highlight.active = false
                }
            }
            PlayerState::ThrowingDice() => (),
        }
    }
}

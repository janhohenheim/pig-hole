use std::fmt::Display;

use crate::player::Player;
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
    Ghost,
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
        self.status == PigStatus::Occupied
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

fn spawn_board(mut commands: Commands) {
    let board_size = Vec3::new(150., 150., 0.);
    let quadrant_translation = Vec2::new(board_size.x / 2., board_size.y / 2.);
    let padding = 20.;
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
                        .spawn_bundle(make_border_bundle(Vec2::new(
                            2. * board_size.x + 3. * padding,
                            2. * board_size.y + 3. * padding,
                        )))
                        .insert(Name::new("Inner border"));

                    parent
                        .spawn_bundle(make_border_bundle(Vec2::new(
                            2. * board_size.x + 4. * padding,
                            2. * board_size.y + 4. * padding,
                        )))
                        .insert(Name::new("Outer border"));
                });
            create_trough(
                parent,
                Trough::new(6, 1),
                Vec2::new(0., 0.),
                Color::GOLD,
                Color::BLACK,
            );

            let top_right_offset = quadrant_offset;
            parent
                .spawn()
                .insert(Name::new("Top right"))
                .insert(GlobalTransform::default())
                .insert(Transform::from_translation(top_right_offset))
                .with_children(|parent| {
                    create_trough(
                        parent,
                        Trough::new(1, 1),
                        -2. * offset_right,
                        Color::WHITE,
                        Color::GRAY,
                    );

                    parent
                        .spawn()
                        .insert(Name::new("Trough 2"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            let inner_color = Color::DARK_GREEN;
                            let outer_color = Color::GREEN;
                            create_trough(
                                parent,
                                Trough::new(2, 1),
                                Vec2::ZERO,
                                outer_color,
                                inner_color,
                            );
                            create_trough(
                                parent,
                                Trough::new(2, 2),
                                2. * offset_right,
                                outer_color,
                                inner_color,
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
                            let outer_color = Color::YELLOW;
                            let inner_color = Color::ORANGE;

                            create_trough(
                                parent,
                                Trough::new(3, 1),
                                2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(3, 2),
                                Vec2::ZERO,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(3, 3),
                                -2. * offset_left,
                                outer_color,
                                inner_color,
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
                            let outer_color = Color::AQUAMARINE;
                            let inner_color = Color::BLUE;

                            create_trough(
                                parent,
                                Trough::new(4, 1),
                                2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(4, 2),
                                2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(4, 3),
                                -2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(4, 4),
                                -2. * offset_left,
                                outer_color,
                                inner_color,
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
                            let outer_color = Color::SALMON;
                            let inner_color = Color::RED;

                            create_trough(
                                parent,
                                Trough::new(5, 1),
                                2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(5, 2),
                                2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(5, 3),
                                -2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(5, 4),
                                -2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_trough(
                                parent,
                                Trough::new(5, 5),
                                Vec2::ZERO,
                                outer_color,
                                inner_color,
                            );
                        });
                });
        });
}

const HOLE_LINE_WIDTH: f32 = 4.0;

fn create_trough(
    parent: &mut ChildBuilder,
    trough: Trough,
    position: Vec2,
    outer_color: Color,
    inner_color: Color,
) {
    parent
        .spawn_bundle(make_trough_bundle(outer_color, inner_color, position))
        .insert(Name::new(format!("Trough {}", trough)))
        .with_children(|parent| {
            let pig = Pig::in_trough(trough);
            parent
                .spawn_bundle(make_pig_bundle())
                .insert(pig)
                .insert(Name::new(format!("Pig {}", pig.trough)))
                .insert(Visibility { is_visible: false })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(make_highlight_bundle())
                        .insert(Name::new(format!("Highlight {}", pig.trough)))
                        .insert(Visibility { is_visible: false })
                        .insert(Highlight { active: false });
                });
        });
}

fn make_highlight_bundle() -> impl Bundle {
    let color = Color::TURQUOISE;
    GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 28.0,
            ..default()
        },
        DrawMode::Fill(FillMode::color(Color::Rgba {
            red: color.r(),
            green: color.g(),
            blue: color.b(),
            alpha: 0.3,
        })),
        Transform::from_xyz(0., 0., -2.),
    )
}

fn make_pig_bundle() -> impl Bundle {
    GeometryBuilder::build_as(
        &shapes::Circle {
            radius: 17.0,
            ..default()
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PINK),
            outline_mode: StrokeMode::new(Color::WHITE, 2.0),
        },
        Transform::from_xyz(0., 0., 2.),
    )
}

fn get_hole_shape() -> impl Geometry {
    shapes::Circle {
        radius: 20.0,
        ..default()
    }
}

fn make_trough_bundle(outer_color: Color, inner_color: Color, transform: Vec2) -> impl Bundle {
    GeometryBuilder::build_as(
        &get_hole_shape(),
        DrawMode::Outlined {
            fill_mode: FillMode::color(inner_color),
            outline_mode: StrokeMode::new(outer_color, HOLE_LINE_WIDTH),
        },
        Transform::from_xyz(transform.x, transform.y, 1.),
    )
}

fn make_border_bundle(extents: Vec2) -> impl Bundle {
    GeometryBuilder::build_as(
        &shapes::Rectangle {
            extents,
            ..default()
        },
        DrawMode::Stroke(StrokeMode::new(Color::BLACK, HOLE_LINE_WIDTH)),
        Transform::default(),
    )
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
            PigStatus::Ghost => {
                visibility.is_visible = true;
                *draw_mode = with_alpha(&draw_mode, 0.5);
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
            crate::player::PlayerState::Selecting(outer_trough_number) => {
                for (pig_entity, &pig) in pig_query.iter() {
                    if pig.trough.group == outer_trough_number && !pig.is_occupied() {
                        for (parent, mut highlight) in highlight_query.iter_mut() {
                            if parent.0 == pig_entity {
                                highlight.active = true;
                            }
                        }
                    }
                }
            }
            crate::player::PlayerState::ThrowingDice() => {
                for (_parent, mut highlight) in highlight_query.iter_mut() {
                    highlight.active = false
                }
            }
        }
    }
}

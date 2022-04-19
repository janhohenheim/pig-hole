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
pub struct PigId {
    pub outer: u8,
    pub inner: u8,
    pub occupied: bool,
}

#[cfg_attr(feature = "dev", derive(Inspectable))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Component)]
pub struct Ghost {
    active: bool,
}

impl PigId {
    pub fn new(outer: u8, inner: u8) -> Self {
        if inner > outer {
            panic!("inner cannot be greater than outer");
        }
        if outer > 6 {
            panic!("outer cannot be greater than 6");
        }

        Self {
            outer,
            inner,
            occupied: false,
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_board))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_pig_visibility)
                    .with_system(update_ghost_visibility)
                    .with_system(activate_ghosts),
            );
        #[cfg(feature = "dev")]
        {
            app.register_inspectable::<PigId>();
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
            create_mound(
                parent,
                PigId::new(6, 1),
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
                    create_mound(
                        parent,
                        PigId::new(1, 1),
                        -2. * offset_right,
                        Color::WHITE,
                        Color::GRAY,
                    );

                    parent
                        .spawn()
                        .insert(Name::new("Mound 2"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            let inner_color = Color::DARK_GREEN;
                            let outer_color = Color::GREEN;
                            create_mound(
                                parent,
                                PigId::new(2, 1),
                                Vec2::ZERO,
                                outer_color,
                                inner_color,
                            );
                            create_mound(
                                parent,
                                PigId::new(2, 2),
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
                        .insert(Name::new("Mound 3"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            let outer_color = Color::YELLOW;
                            let inner_color = Color::ORANGE;

                            create_mound(
                                parent,
                                PigId::new(3, 1),
                                2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(3, 2),
                                Vec2::ZERO,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(3, 3),
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
                        .insert(Name::new("Mound 4"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            let outer_color = Color::AQUAMARINE;
                            let inner_color = Color::BLUE;

                            create_mound(
                                parent,
                                PigId::new(4, 1),
                                2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(4, 2),
                                2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(4, 3),
                                -2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(4, 4),
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
                        .insert(Name::new("Mound 5"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            let outer_color = Color::SALMON;
                            let inner_color = Color::RED;

                            create_mound(
                                parent,
                                PigId::new(5, 1),
                                2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(5, 2),
                                2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(5, 3),
                                -2. * offset_right,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(5, 4),
                                -2. * offset_left,
                                outer_color,
                                inner_color,
                            );

                            create_mound(
                                parent,
                                PigId::new(5, 5),
                                Vec2::ZERO,
                                outer_color,
                                inner_color,
                            );
                        });
                });
        });
}

const HOLE_LINE_WIDTH: f32 = 4.0;

fn create_mound(
    parent: &mut ChildBuilder,
    pig_id: PigId,
    position: Vec2,
    outer_color: Color,
    inner_color: Color,
) {
    parent
        .spawn_bundle(make_mound_bundle(outer_color, inner_color, position))
        .insert(Name::new(format!(
            "Mound {}.{}",
            pig_id.outer, pig_id.inner
        )))
        .with_children(|parent| {
            parent
                .spawn_bundle(make_pig_bundle())
                .insert(pig_id)
                .insert(Name::new(format!("Pig {}.{}", pig_id.outer, pig_id.inner)))
                .insert(Visibility { is_visible: false })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(make_ghost_bundle())
                        .insert(Name::new(format!(
                            "Ghost {}.{}",
                            pig_id.outer, pig_id.inner
                        )))
                        .insert(Visibility { is_visible: false })
                        .insert(Ghost { active: false });
                });
        });
}

fn make_ghost_bundle() -> impl Bundle {
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

fn make_mound_bundle(outer_color: Color, inner_color: Color, transform: Vec2) -> impl Bundle {
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

fn update_pig_visibility(mut pig_id_query: Query<(&mut PigId, &mut Visibility)>) {
    for (mut pig_id, mut visibility) in pig_id_query.iter_mut() {
        if pig_id.outer == 6 {
            pig_id.occupied = false;
        }
        visibility.is_visible = pig_id.occupied;
    }
}

fn update_ghost_visibility(mut ghost_query: Query<(&Ghost, &mut Visibility)>) {
    for (ghost, mut visibility) in ghost_query.iter_mut() {
        visibility.is_visible = ghost.active;
    }
}

fn activate_ghosts(
    pig_id_query: Query<(Entity, &PigId)>,
    mut ghost_query: Query<(&Parent, &mut Ghost)>,
    player_query: Query<&Player>,
) {
    for player in player_query.iter() {
        match player.state {
            crate::player::PlayerState::Selecting(outer_mould_index) => {
                for (pig_entity, &pig_id) in pig_id_query.iter() {
                    if pig_id.outer == outer_mould_index {
                        for (parent, mut ghost) in ghost_query.iter_mut() {
                            if parent.0 == pig_entity {
                                ghost.active = true;
                            }
                        }
                    }
                }
            }
            crate::player::PlayerState::ThrowingDice() => {
                for (_parent, mut ghost) in ghost_query.iter_mut() {
                    ghost.active = false
                }
            }
        }
    }
}

use crate::GameState;
use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::Inspectable;
#[cfg(feature = "dev")]
use bevy_inspector_egui::RegisterInspectable;
use bevy_prototype_lyon::prelude::*;

#[cfg_attr(feature = "dev", derive(Inspectable))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Component)]
pub struct MouldId {
    pub outer: u8,
    pub inner: u8,
}

impl MouldId {
    pub fn new(outer: u8, inner: u8) -> Self {
        if inner > outer {
            panic!("inner cannot be greater than outer");
        }
        if outer > 6 {
            panic!("outer cannot be greater than 6");
        }

        Self { outer, inner }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_board));
        #[cfg(feature = "dev")]
        {
            app.register_inspectable::<MouldId>();
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

            parent
                .spawn_bundle(make_hole_bundle(Color::GOLD, (0., 0.)))
                .insert(Name::new("Pig hole"))
                .insert(MouldId::new(6, 1));

            let top_right_offset = quadrant_offset;
            parent
                .spawn()
                .insert(Name::new("Top right"))
                .insert(GlobalTransform::default())
                .insert(Transform::from_translation(top_right_offset))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(make_mound_bundle(Color::WHITE, -2. * offset_right))
                        .insert(Name::new("Mound 1"));

                    parent
                        .spawn()
                        .insert(Name::new("Mound 2"))
                        .insert(GlobalTransform::default())
                        .insert(Transform::default())
                        .with_children(|parent| {
                            let color = Color::DARK_GREEN;
                            parent
                                .spawn_bundle(make_mound_bundle(color, Vec2::ZERO))
                                .insert(MouldId::new(2, 1))
                                .insert(Name::new("Mound 2.1"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, 2. * offset_right))
                                .insert(MouldId::new(2, 2))
                                .insert(Name::new("Mound 2.2"));
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
                            let color = Color::YELLOW;
                            parent
                                .spawn_bundle(make_mound_bundle(color, 2. * offset_left))
                                .insert(MouldId::new(3, 1))
                                .insert(Name::new("Mound 3.1"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, Vec2::ZERO))
                                .insert(MouldId::new(3, 2))
                                .insert(Name::new("Mound 3.2"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, -2. * offset_left))
                                .insert(MouldId::new(3, 3))
                                .insert(Name::new("Mound 3.3"));
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
                            let color = Color::BLUE;
                            parent
                                .spawn_bundle(make_mound_bundle(color, 2. * offset_right))
                                .insert(MouldId::new(4, 1))
                                .insert(Name::new("Mound 4.1"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, 2. * offset_left))
                                .insert(MouldId::new(4, 2))
                                .insert(Name::new("Mound 4.2"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, -2. * offset_right))
                                .insert(MouldId::new(4, 3))
                                .insert(Name::new("Mound 4.3"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, -2. * offset_left))
                                .insert(MouldId::new(4, 4))
                                .insert(Name::new("Mound 4.4"));
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
                            let color = Color::RED;
                            parent
                                .spawn_bundle(make_mound_bundle(color, 2. * offset_right))
                                .insert(MouldId::new(5, 1))
                                .insert(Name::new("Mound 5.1"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, 2. * offset_left))
                                .insert(MouldId::new(5, 2))
                                .insert(Name::new("Mound 5.2"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, -2. * offset_right))
                                .insert(MouldId::new(5, 3))
                                .insert(Name::new("Mound 5.3"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, -2. * offset_left))
                                .insert(MouldId::new(5, 4))
                                .insert(Name::new("Mound 5.4"));

                            parent
                                .spawn_bundle(make_mound_bundle(color, Vec2::ZERO))
                                .insert(MouldId::new(5, 5))
                                .insert(Name::new("Mound 5.5"));
                        });
                });
        });
}

const HOLE_LINE_WIDTH: f32 = 4.0;

fn make_hole_bundle(outline_color: Color, position: (f32, f32)) -> impl Bundle {
    GeometryBuilder::build_as(
        &get_hole_shape(),
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::BLACK),
            outline_mode: StrokeMode::new(outline_color, HOLE_LINE_WIDTH),
        },
        Transform::from_xyz(position.0, position.1, 1.0),
    )
}

fn get_hole_shape() -> impl Geometry {
    shapes::Circle {
        radius: 20.0,
        center: Vec2::ZERO,
    }
}

fn make_mound_bundle(fill_color: Color, transform: Vec2) -> impl Bundle {
    GeometryBuilder::build_as(
        &get_hole_shape(),
        DrawMode::Outlined {
            fill_mode: FillMode::color(fill_color),
            outline_mode: StrokeMode::new(Color::BLACK, HOLE_LINE_WIDTH),
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

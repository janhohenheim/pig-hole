use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BoardAssetCreator {}

impl BoardAssetCreator {
    pub fn get_trough_for_group(&self, group: u8) -> ShapeBundle {
        match group {
            1 => make_trough_bundle(Color::GRAY, Color::WHITE),
            2 => make_trough_bundle(Color::DARK_GREEN, Color::GREEN),
            3 => make_trough_bundle(Color::ORANGE, Color::YELLOW),
            4 => make_trough_bundle(Color::BLUE, Color::AZURE),
            5 => make_trough_bundle(Color::RED, Color::SALMON),
            6 => make_trough_bundle(Color::BLACK, Color::GOLD),
            group => panic!("Invalid group: {}", group),
        }
    }

    pub fn get_pig(&self) -> ShapeBundle {
        make_pig_bundle()
    }

    pub fn get_pig_aabb(&self) -> Vec2 {
        Vec2::splat(2.0 * PIG_RADIUS)
    }

    pub fn get_pig_collection_padding(&self) -> f32 {
        5.0
    }

    pub fn get_highlight(&self) -> ShapeBundle {
        make_highlight_bundle()
    }

    pub fn get_border(&self) -> ShapeBundle {
        let extents = self.get_board_extents();
        let padding = self.get_board_padding();
        make_border_bundle(extents, padding)
    }

    pub fn get_board_extents(&self) -> Vec2 {
        Vec2::new(150.0, 150.0)
    }

    pub fn get_board_padding(&self) -> f32 {
        20.0
    }
}

const HOLE_LINE_WIDTH: f32 = 4.0;

fn make_trough_bundle(fill_color: Color, outline_color: Color) -> ShapeBundle {
    GeometryBuilder::build_as(
        &get_hole_shape(),
        DrawMode::Outlined {
            fill_mode: FillMode::color(fill_color),
            outline_mode: StrokeMode::new(outline_color, HOLE_LINE_WIDTH),
        },
        Transform::default(),
    )
}

fn get_hole_shape() -> impl Geometry {
    shapes::Circle {
        radius: 20.0,
        ..default()
    }
}

const PIG_RADIUS: f32 = 17.0;

fn make_pig_bundle() -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::Circle {
            radius: PIG_RADIUS,
            ..default()
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::PINK),
            outline_mode: StrokeMode::new(Color::WHITE, 2.0),
        },
        Transform::from_xyz(0., 0., 2.),
    )
}

fn make_highlight_bundle() -> ShapeBundle {
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

fn make_border_bundle(extents: Vec2, padding: f32) -> ShapeBundle {
    let mode = DrawMode::Stroke(StrokeMode::new(Color::BLACK, HOLE_LINE_WIDTH));
    GeometryBuilder::new()
        .add(&make_border_part_bundle(Vec2::new(
            2. * extents.x + 3. * padding,
            2. * extents.y + 3. * padding,
        )))
        .add(&make_border_part_bundle(Vec2::new(
            2. * extents.x + 4. * padding,
            2. * extents.y + 4. * padding,
        )))
        .build(mode, Transform::default())
}

fn make_border_part_bundle(extents: Vec2) -> impl Geometry {
    shapes::Rectangle {
        extents,
        ..default()
    }
}

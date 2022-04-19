use crate::GameState;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_board));
    }
}

fn spawn_board(mut commands: Commands) {
    commands
        .spawn()
        .insert(Name::new("Board"))
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(make_hole_bundle(Color::SALMON, (0., 0.)))
                .insert(Name::new("Six Hole"));
        });
}

trait CommandExtension<'w, 's, 'a> {
    fn spawn_hole(
        &'a mut self,
        outline_color: Color,
        position: (f32, f32),
    ) -> EntityCommands<'w, 's, '_>;
}

fn make_hole_bundle(outline_color: Color, position: (f32, f32)) -> impl Bundle {
    GeometryBuilder::build_as(
        &get_hole_shape(),
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::BLACK),
            outline_mode: StrokeMode::new(outline_color, 4.0),
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

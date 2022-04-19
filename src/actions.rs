use crate::{board::PigId, GameState};
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_mouse_actions),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub selected_mould: Option<PigId>,
}

fn set_mouse_actions(
    mut actions: ResMut<Actions>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mould_position_query: Query<(&GlobalTransform, &PigId)>,
) {
    let window = windows.get_primary().expect("No primary window found");
    if mouse_input.just_pressed(MouseButton::Left) {
        actions.selected_mould = if let Some(position) = window.cursor_position() {
            let world_position = Vec2::new(
                position.x - window.width() / 2.,
                position.y - window.height() / 2.,
            );

            let mut closest_mould = None;
            for (transform, pig_id) in mould_position_query.iter() {
                const RADIUS: f32 = 20.0;
                if world_position.x <= transform.translation.x + RADIUS
                    && world_position.x >= transform.translation.x - RADIUS
                    && world_position.y <= transform.translation.y + RADIUS
                    && world_position.y >= transform.translation.y - RADIUS
                {
                    closest_mould = Some(*pig_id);
                    break;
                }
            }
            closest_mould
        } else {
            None
        }
    }
}

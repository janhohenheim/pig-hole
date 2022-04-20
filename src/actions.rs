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
    pub hovered_mould: Option<PigId>,
    pub selected_mould: Option<PigId>,
}

fn set_mouse_actions(
    mut actions: ResMut<Actions>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mould_position_query: Query<(&GlobalTransform, &PigId)>,
) {
    if actions.selected_mould.is_some() {
        actions.selected_mould = None;
    }
    let window = windows.get_primary().expect("No primary window found");
    if mouse_input.just_pressed(MouseButton::Left) {
        actions.selected_mould = get_pig_id_under_cursor(mould_position_query, window);
    } else {
        actions.hovered_mould = get_pig_id_under_cursor(mould_position_query, window);
    }
}

fn get_cursor_world_position(window: &Window) -> Option<Vec2> {
    window.cursor_position().map(|position| {
        Vec2::new(
            position.x - window.width() / 2.,
            position.y - window.height() / 2.,
        )
    })
}

fn get_pig_id_under_cursor(
    mould_position_query: Query<(&GlobalTransform, &PigId)>,
    window: &Window,
) -> Option<PigId> {
    if let Some(position) = get_cursor_world_position(window) {
        for (transform, pig_id) in mould_position_query.iter() {
            const RADIUS: f32 = 20.0;
            if position.x <= transform.translation.x + RADIUS
                && position.x >= transform.translation.x - RADIUS
                && position.y <= transform.translation.y + RADIUS
                && position.y >= transform.translation.y - RADIUS
            {
                return Some(*pig_id);
            }
        }
        None
    } else {
        None
    }
}

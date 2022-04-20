use crate::actions::Actions;
use crate::board::PigId;
use crate::board::PigStatus;
use crate::GameState;
use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::Inspectable;
#[cfg(feature = "dev")]
use bevy_inspector_egui::RegisterInspectable;

pub struct PlayerPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
#[cfg_attr(feature = "dev", derive(Inspectable))]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "dev", derive(Inspectable))]
pub enum PlayerState {
    Selecting(u8),
    ThrowingDice(),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OuterMouldIndex(u8);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_camera)
                .with_system(spawn_player),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(place_pig));

        #[cfg(feature = "dev")]
        {
            app.register_inspectable::<Player>()
                .register_inspectable::<PlayerState>();
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Name::new("Camera"));
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player {
            state: PlayerState::Selecting(3),
        })
        .insert(Name::new("Player"));
}

fn place_pig(
    mut pig_query: Query<&mut PigId>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Player>,
) {
    for mut player in player_query.iter_mut() {
        match player.state {
            PlayerState::Selecting(outer_mould_index) => {
                if let Some(selected_pig_id) = actions.selected_mould {
                    for mut pig_id in pig_query.iter_mut() {
                        if *pig_id == selected_pig_id
                            && selected_pig_id.outer == outer_mould_index
                            && selected_pig_id.status != PigStatus::Occupied
                        {
                            pig_id.status = PigStatus::Occupied;
                            player.state = PlayerState::ThrowingDice();
                        }
                    }
                }
            }
            PlayerState::ThrowingDice() => (),
        }
    }
}

use crate::actions::Actions;
use crate::board::PigId;
use crate::board::PigStatus;
use crate::GameState;
use bevy::log;
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
                    if let Some(mut pig) = find_mut_pig_id(&selected_pig_id, &mut pig_query) {
                        if is_valid_for_placement(&pig, outer_mould_index) {
                            pig.status = PigStatus::Occupied;
                            player.state = PlayerState::ThrowingDice();
                            clear_ghosts(&mut pig_query);
                        }
                    }
                } else if let Some(hovered_pig_id) = actions.hovered_mould {
                    if let Some(mut pig) = find_mut_pig_id(&hovered_pig_id, &mut pig_query) {
                        if is_valid_for_placement(&pig, outer_mould_index) {
                            pig.status = PigStatus::Ghost;
                            log::info!("Ghost pig: {:?}", hovered_pig_id);
                        }
                    }
                    clear_ghosts_except(&mut pig_query, &hovered_pig_id);
                } else {
                    clear_ghosts(&mut pig_query);
                }
            }
            PlayerState::ThrowingDice() => (),
        }
    }
}

fn find_mut_pig_id<'a>(
    needle: &PigId,
    haystack: &'a mut Query<&mut PigId>,
) -> Option<Mut<'a, PigId>> {
    haystack
        .iter_mut()
        .filter(|pig_id| **pig_id == *needle)
        .next()
}

fn is_valid_for_placement(pig_id: &PigId, outer_mould_index: u8) -> bool {
    pig_id.outer == outer_mould_index && pig_id.status != PigStatus::Occupied
}

fn clear_ghosts(pig_query: &mut Query<&mut PigId>) {
    for mut pig in pig_query
        .iter_mut()
        .filter(|pig| pig.status == PigStatus::Ghost)
    {
        pig.status = PigStatus::Empty;
    }
}

fn clear_ghosts_except(pig_query: &mut Query<&mut PigId>, exception: &PigId) {
    for mut pig in pig_query.iter_mut().filter(|pig| {
        pig.status == PigStatus::Ghost
            && pig.outer != exception.outer
            && pig.inner != exception.inner
    }) {
        pig.status = PigStatus::Empty;
    }
}

use crate::actions::Actions;
use crate::board::Pig;
use crate::board::PigStatus;
use crate::GameState;
use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::Inspectable;
#[cfg(feature = "dev")]
use bevy_inspector_egui::RegisterInspectable;
use rand::Rng;
pub struct PlayerPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
#[cfg_attr(feature = "dev", derive(Inspectable))]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "dev", derive(Inspectable))]
pub enum PlayerState {
    PlacingInGroup(u8),
    CollectingGroup(u8),
    Thinking(),
    ThrowingDice(),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OuterTroughIndex(u8);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_camera)
                .with_system(spawn_player),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(select_pig)
                .with_system(throw_dice),
        );

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
            state: PlayerState::Thinking(),
        })
        .insert(Name::new("Player"));
}

fn select_pig(
    mut pig_query: Query<&mut Pig>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Player>,
) {
    for mut player in player_query.iter_mut() {
        match player.state {
            PlayerState::PlacingInGroup(group) => {
                if let Some(selected_pig) = actions.selected_pig {
                    if let Some(mut pig) = find_mut_pig(&selected_pig, &mut pig_query) {
                        if is_valid_for_placement(&pig, group) {
                            pig.status = PigStatus::Occupied;
                            player.state = PlayerState::Thinking();
                            clear_ghosts(&mut pig_query);
                        }
                    }
                } else if let Some(hovered_pig) = actions.hovered_trough {
                    if let Some(mut pig) = find_mut_pig(&hovered_pig, &mut pig_query) {
                        if is_valid_for_placement(&pig, group) {
                            pig.status = PigStatus::PlacementGhost;
                        }
                    }
                    clear_ghosts_except(&mut pig_query, &hovered_pig);
                } else {
                    clear_ghosts(&mut pig_query);
                }
            }
            PlayerState::CollectingGroup(group) => {
                if let Some(selected_pig) = actions.selected_pig {
                    if selected_pig.trough.group != group {
                        return;
                    }
                    for mut pig in pig_query.iter_mut() {
                        if pig.trough.group == group {
                            pig.status = PigStatus::Empty;
                            player.state = PlayerState::Thinking();
                        }
                    }
                } else if let Some(hovered_pig) = actions.hovered_trough {
                    if hovered_pig.trough.group != group {
                        return;
                    }
                    if let Some(mut pig) = find_mut_pig(&hovered_pig, &mut pig_query) {
                        if is_valid_for_placement(&pig, group) {
                            pig.status = PigStatus::PlacementGhost;
                        }
                    }
                    for mut pig in pig_query.iter_mut() {
                        if pig.trough.group == group {
                            pig.status = PigStatus::RemovalGhost;
                        }
                    }
                } else {
                    clear_ghosts(&mut pig_query);
                }
            }
            PlayerState::Thinking() | PlayerState::ThrowingDice() => (),
        }
    }
}

fn throw_dice(mut player_query: Query<&mut Player>, pig_query: Query<&Pig>) {
    for mut player in player_query.iter_mut() {
        match player.state {
            PlayerState::ThrowingDice() => {
                let mut rng = rand::thread_rng();
                let roll = rng.gen_range(1..=6);
                player.state = if is_group_full(&pig_query, roll) {
                    PlayerState::CollectingGroup(roll)
                } else {
                    PlayerState::PlacingInGroup(roll)
                };
            }
            _ => (),
        }
    }
}

fn is_group_full(pig_query: &Query<&Pig>, group: u8) -> bool {
    for pig in pig_query.iter() {
        if !pig.is_occupied() && pig.trough.group == group {
            return false;
        }
    }
    true
}

fn find_mut_pig<'a>(needle: &Pig, haystack: &'a mut Query<&mut Pig>) -> Option<Mut<'a, Pig>> {
    haystack.iter_mut().find(|pig| **pig == *needle)
}

fn is_valid_for_placement(pig: &Pig, group: u8) -> bool {
    pig.trough.group == group && !pig.is_occupied()
}

fn clear_ghosts(pig_query: &mut Query<&mut Pig>) {
    for mut pig in pig_query.iter_mut() {
        match pig.status {
            PigStatus::PlacementGhost => pig.status = PigStatus::Empty,
            PigStatus::RemovalGhost => pig.status = PigStatus::Occupied,
            _ => (),
        }
    }
}

fn clear_ghosts_except(pig_query: &mut Query<&mut Pig>, exception: &Pig) {
    for mut pig in pig_query
        .iter_mut()
        .filter(|pig| pig.status == PigStatus::PlacementGhost && pig.trough != exception.trough)
    {
        pig.status = PigStatus::Empty;
    }
}

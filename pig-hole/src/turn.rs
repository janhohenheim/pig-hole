use crate::{player::Player, GameState};
use bevy::prelude::*;
use std::fmt::Display;
pub struct TurnPlugin;
#[cfg(feature = "dev")]
use bevy_inspector_egui::Inspectable;

// This plugin is responsible to control the game audio
impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_current_player),
        )
        .add_event::<TurnChangeEvent>();
    }
}

#[derive(Debug, Eq, Default, PartialEq, Hash, Component)]
#[cfg_attr(feature = "dev", derive(Inspectable))]
pub struct IsOnTurn;

pub struct TurnChangeEvent;

#[derive(Debug, Eq, PartialEq, Hash, Component)]
pub struct Turn {
    number: usize,
    player_order: Vec<Entity>,
    current_player_index: usize,
}

impl Turn {
    pub fn new(player_order: Vec<Entity>) -> Self {
        Self {
            number: 1,
            player_order,
            current_player_index: 0,
        }
    }
    pub fn get_min_actions(&self) -> Option<usize> {
        match self.number {
            0 => panic!("Turn 0 is invalid"),
            n if n <= 2 => Some(n),
            _ => None,
        }
    }

    pub fn start_next_players_turn(&mut self) -> Entity {
        self.current_player_index += 1;
        if self.current_player_index >= self.player_order.len() {
            self.current_player_index = 0;
            self.number += 1;
        }
        self.get_current_player()
    }

    pub fn get_current_player(&self) -> Entity {
        self.player_order[self.current_player_index]
    }

    pub fn get_turn_number(&self) -> usize {
        self.number
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Turn {}, current player: {}/{}",
            self.number,
            self.current_player_index + 1,
            self.player_order.len()
        )
    }
}

fn set_current_player(
    mut commands: Commands,
    mut turn: ResMut<Turn>,
    player_query: Query<Entity, With<Player>>,
    mut turn_change_event_reader: EventReader<TurnChangeEvent>,
) {
    for _event in turn_change_event_reader.iter() {
        let current_player = turn.start_next_players_turn();
        for entity in player_query.iter() {
            let mut player = commands.entity(entity);
            if entity == current_player {
                player.insert(IsOnTurn);
            } else {
                player.remove::<IsOnTurn>();
            }
        }
    }
}

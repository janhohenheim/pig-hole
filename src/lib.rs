mod actions;
mod audio;
mod board;
mod dev;
mod ingame_menu;
mod loading;
mod menu;
mod pig_collection;
mod player;
mod player_creation;
mod turn;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::board::BoardPlugin;
use crate::dev::DevPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::pig_collection::PigCollectionPlugin;
use crate::player::PlayerPlugin;
use crate::player_creation::PlayerCreationPlugin;
use crate::turn::TurnPlugin;

use bevy::app::App;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    JoiningLobby,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(BoardPlugin)
            .add_plugin(PigCollectionPlugin)
            .add_plugin(TurnPlugin)
            .add_plugin(PlayerCreationPlugin)
            .add_plugin(DevPlugin);
    }
}

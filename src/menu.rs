use self::joining_lobby::JoiningLobbyPlugin;
use self::main_menu::MainMenuPlugin;
use bevy::prelude::*;

mod joining_lobby;
mod main_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin)
            .add_plugin(JoiningLobbyPlugin);
    }
}

use bevy::prelude::*;

pub struct WaitingForPlayersPlugin;

impl Plugin for WaitingForPlayersPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Clone, PartialEq)]
pub enum WaitingForPlayersSubMenu {
    Main(ViewModel),
}

impl Default for WaitingForPlayersSubMenu {
    fn default() -> Self {
        WaitingForPlayersSubMenu::Main(default())
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct ViewModel {}

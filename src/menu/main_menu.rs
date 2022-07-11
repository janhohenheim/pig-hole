use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct MainMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(show_menu));
    }
}

fn show_menu(mut egui_ctx: ResMut<EguiContext>, mut state: ResMut<State<GameState>>) {
    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(300.0);
            ui.heading("Pig Hole");
            if ui.button("Play").clicked() {
                state.set(GameState::JoiningLobby).unwrap()
            }
        });
    });
}

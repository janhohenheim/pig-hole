use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::SubMenu;

pub struct MainMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(show_menu));
    }
}

fn show_menu(mut egui_ctx: ResMut<EguiContext>, mut sub_menu: ResMut<SubMenu>) {
    if *sub_menu != SubMenu::Main {
        return;
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(200.0);
            ui.push_id("Pig Hole", |ui| {
                ui.heading("Pig Hole");
            });
            ui.add_space(100.0);
            let layout = egui::Layout::centered_and_justified(ui.layout().main_dir());
            ui.allocate_ui_with_layout(egui::Vec2::new(300.0, 0.0), layout, |ui| {
                if ui.button("Browse Games").clicked() {
                    *sub_menu = SubMenu::BrowseLobbies(default())
                }
                if ui.button("Host Game").clicked() {
                    *sub_menu = SubMenu::CreateLobby(default())
                }
            });
        });
    });
}

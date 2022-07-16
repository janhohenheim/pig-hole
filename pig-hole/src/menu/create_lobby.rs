use crate::{networking, GameState};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool, tasks::Task};
use bevy_egui::{egui, EguiContext};

mod waiting_for_players;
use renet::RenetClient;
use waiting_for_players::{WaitingForPlayersPlugin, WaitingForPlayersSubMenu};

use super::SubMenu;

pub struct CreateLobbyPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for CreateLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(show_menu)
                .with_system(go_back)
                .with_system(create_lobby),
        );
        app.add_plugin(WaitingForPlayersPlugin);
    }
}

pub enum CreateLobbySubMenu {
    Main(ViewModel),
    WaitingForPlayers(WaitingForPlayersSubMenu),
}

impl Default for CreateLobbySubMenu {
    fn default() -> Self {
        CreateLobbySubMenu::Main(default())
    }
}
#[derive(Default)]
pub struct ViewModel {
    player_name: String,
    lobby_name: String,
    back: bool,
    create_lobby: bool,
    client_task: Option<Task<RenetClient>>,
}

fn go_back(mut egui_ctx: ResMut<EguiContext>, mut sub_menu: ResMut<SubMenu>) {
    let view_model = match &mut *sub_menu {
        SubMenu::CreateLobby(CreateLobbySubMenu::Main(view_model)) => view_model,
        _ => return,
    };
    if view_model.back {
        *sub_menu = SubMenu::Main;
    }
}

fn create_lobby(
    mut egui_ctx: ResMut<EguiContext>,
    mut sub_menu: ResMut<SubMenu>,
    pool: Res<AsyncComputeTaskPool>,
) {
    let view_model = match &mut *sub_menu {
        SubMenu::CreateLobby(CreateLobbySubMenu::Main(view_model)) => view_model,
        _ => return,
    };
    if !view_model.create_lobby {
        return;
    }

    let username = &view_model.player_name;
    let lobby_name = &view_model.lobby_name;
    let task = pool.spawn(async move { networking::create_lobby(username, lobby_name).await });
    view_model.client_task = Some(task);
}

fn show_menu(mut egui_ctx: ResMut<EguiContext>, mut sub_menu: ResMut<SubMenu>) {
    let view_model = match &mut *sub_menu {
        SubMenu::CreateLobby(CreateLobbySubMenu::Main(view_model)) => view_model,
        _ => return,
    };

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        let center = ui.available_size() / 2.0;
        ui.allocate_ui_at_rect(
            egui::Rect::from_center_size(center.to_pos2(), egui::Vec2::new(400.0, 400.0)),
            |ui| {
                ui.push_id("Creating Server", |ui| {
                    ui.heading("Creating Server");
                });
                ui.add_space(100.0);
                ui.horizontal(|ui| {
                    ui.label("Player Name: ");
                    ui.text_edit_singleline(&mut view_model.player_name);
                });
                ui.horizontal(|ui| {
                    ui.label("Lobby Name: ");
                    ui.text_edit_singleline(&mut view_model.lobby_name);
                });
                ui.horizontal(|ui| {
                    if ui.button("Back").clicked() {
                        view_model.back = true;
                    }
                    let enabled = !view_model.player_name.is_empty()
                        && !view_model.lobby_name.is_empty()
                        && !view_model.client_task.is_some();
                    if ui
                        .add_enabled(enabled, egui::Button::new("Create Lobby"))
                        .clicked()
                    {
                        view_model.create_lobby = true;
                    }
                });
                if view_model.client_task.is_some() {
                    ui.add_space(100.0);
                    ui.horizontal(|ui| {
                        ui.label("Creating Lobby...");
                        ui.spinner();
                    });
                }
            },
        );
    });
}

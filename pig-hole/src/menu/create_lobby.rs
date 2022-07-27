use super::SubMenu;
use crate::{networking, GameState};
use async_channel::{Receiver, Sender};
use bevy::{log, prelude::*, tasks::AsyncComputeTaskPool};
use bevy_egui::{egui, EguiContext};
use renet::RenetClient;
use waiting_for_players::{WaitingForPlayersPlugin, WaitingForPlayersSubMenu};

mod waiting_for_players;

pub struct CreateLobbyPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for CreateLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(show_menu)
                .with_system(go_back)
                .with_system(create_lobby)
                .with_system(poll_lobby_creation),
        );
        let (task_sender, task_receiver) = async_channel::unbounded::<RenetClient>();
        app.insert_resource(task_sender);
        app.insert_resource(task_receiver);
        app.add_plugin(WaitingForPlayersPlugin);
    }
}

#[derive(PartialEq, Clone)]
pub enum CreateLobbySubMenu {
    Main(ViewModel),
    WaitingForPlayers(WaitingForPlayersSubMenu),
}

impl Default for CreateLobbySubMenu {
    fn default() -> Self {
        CreateLobbySubMenu::Main(default())
    }
}
#[derive(Default, PartialEq, Clone)]
pub struct ViewModel {
    player_name: String,
    lobby_name: String,
    back: bool,
    lobby_creation_state: LobbyCreationState,
}

#[derive(Eq, PartialEq, Clone)]
pub enum LobbyCreationState {
    None,
    Requested,
    Creating,
}

impl Default for LobbyCreationState {
    fn default() -> Self {
        Self::None
    }
}

fn go_back(commands: Commands, mut sub_menu: ResMut<SubMenu>) {
    let view_model = match &mut *sub_menu {
        SubMenu::CreateLobby(CreateLobbySubMenu::Main(view_model)) => view_model,
        _ => return,
    };
    if view_model.back {
        *sub_menu = SubMenu::Main;
    }
}

fn create_lobby(
    mut sub_menu: ResMut<SubMenu>,
    task_pool: Res<AsyncComputeTaskPool>,
    task_sender: Res<Sender<RenetClient>>,
) {
    let view_model = match &mut *sub_menu {
        SubMenu::CreateLobby(CreateLobbySubMenu::Main(view_model)) => view_model,
        _ => return,
    };
    if !matches!(
        view_model.lobby_creation_state,
        LobbyCreationState::Requested
    ) {
        return;
    }

    let username = view_model.player_name.clone();
    let lobby_name = view_model.lobby_name.clone();
    // Source(s):
    // https://github.com/bevyengine/bevy/discussions/3351
    // ->
    // https://github.com/mvlabat/muddle-run/blob/b02374bf90f29a246c39d89ebf35ba49f53865b4/libs/shared_lib/src/game/level.rs#L161
    let inner_task_sender = task_sender.clone();
    task_pool
        .spawn(async move {
            let client = networking::create_lobby(&username, &lobby_name).await;
            inner_task_sender.send(client).await.unwrap();
        })
        .detach();

    view_model.lobby_creation_state = LobbyCreationState::Creating;
}

fn poll_lobby_creation(
    mut commands: Commands,
    task_receiver: Res<Receiver<RenetClient>>,
    mut sub_menu: ResMut<SubMenu>,
) {
    if let Ok(client) = task_receiver.try_recv() {
        if client.is_connected() {
            *sub_menu = SubMenu::CreateLobby(CreateLobbySubMenu::WaitingForPlayers(default()));
            commands.insert_resource(client);
        } else {
            log::error!("Client connection failed!");
        }
    }
}

fn clean_up(mut commands: Commands) {
    commands.remove_resource::<Receiver<RenetClient>>();
    commands.remove_resource::<Sender<RenetClient>>();
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
                        && matches!(view_model.lobby_creation_state, LobbyCreationState::None);
                    if ui
                        .add_enabled(enabled, egui::Button::new("Create Lobby"))
                        .clicked()
                    {
                        view_model.lobby_creation_state = LobbyCreationState::Requested;
                    }
                });
                if !matches!(view_model.lobby_creation_state, LobbyCreationState::None) {
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

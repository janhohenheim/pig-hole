use super::SubMenu;
use crate::{networking, GameState};
use bevy::{log, prelude::*, tasks::AsyncComputeTaskPool};
use bevy_egui::{egui, EguiContext};
use renet::RenetClient;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use waiting_for_players::{WaitingForPlayersPlugin, WaitingForPlayersSubMenu};
mod waiting_for_players;
use networking::Worker;

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
                .with_system(poll_lobby_creation)
                .with_system(fetch_data),
        )
        .add_startup_system(setup_client_fetch);
        let (task_sender, task_receiver) = unbounded_channel::<RenetClient>();
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

type ClientGetter = Worker<(String, String), RenetClient>;

fn go_back(mut sub_menu: ResMut<SubMenu>) {
    let view_model = match &mut *sub_menu {
        SubMenu::CreateLobby(CreateLobbySubMenu::Main(view_model)) => view_model,
        _ => return,
    };
    if view_model.back {
        *sub_menu = SubMenu::Main;
    }
}

fn setup_client_fetch(mut commands: Commands, task_pool: Res<AsyncComputeTaskPool>) {
    let fetcher = ClientGetter::spawn(&task_pool, |mut receiver, sender| async move {
        loop {
            if let Some((username, lobby_name)) = receiver.recv().await {
                while receiver.try_recv().is_ok() {} // Empty out buffer
                let client = networking::create_lobby(&username, &lobby_name).await;
                match sender.send(client) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
        }
    });
    commands.insert_resource(fetcher);
}

fn create_lobby(mut sub_menu: ResMut<SubMenu>, client_fetcher: Option<ResMut<ClientGetter>>) {
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

    request_data(client_fetcher, username, lobby_name);

    view_model.lobby_creation_state = LobbyCreationState::Creating;
}

fn poll_lobby_creation(
    mut commands: Commands,
    mut task_receiver: ResMut<UnboundedReceiver<RenetClient>>,
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

fn request_data(data_fetcher: Option<ResMut<ClientGetter>>, username: String, lobby_name: String) {
    if let Some(fetcher) = data_fetcher {
        fetcher.input.send((username, lobby_name)).unwrap();
    }
}

fn fetch_data(mut commands: Commands, data_fetcher: Option<ResMut<ClientGetter>>) {
    if let Some(mut fetcher) = data_fetcher {
        while let Ok(client) = fetcher.output.try_recv() {
            commands.insert_resource(client);
        }
    }
}

fn clean_up(mut commands: Commands) {
    commands.remove_resource::<UnboundedReceiver<RenetClient>>();
    commands.remove_resource::<UnboundedSender<RenetClient>>();
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

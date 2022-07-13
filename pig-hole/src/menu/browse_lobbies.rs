use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText},
    EguiContext,
};

mod waiting_for_players;
use egui_extras::{self, Size, *};
use waiting_for_players::{WaitingForPlayersPlugin, WaitingForPlayersSubMenu};

use super::SubMenu;

pub struct BrowseLobbiesPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for BrowseLobbiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(show_menu));
        app.add_plugin(WaitingForPlayersPlugin);
    }
}

#[derive(Clone, PartialEq)]
pub enum BrowseLobbiesSubMenu {
    Main(ViewModel),
    WaitingForPlayers(WaitingForPlayersSubMenu),
}

impl Default for BrowseLobbiesSubMenu {
    fn default() -> Self {
        Self::Main(default())
    }
}
#[derive(Clone, PartialEq, Default)]
pub struct ViewModel {
    player_name: String,
    back: bool,
    join_lobby: Option<String>,
    player_name_empty_warning: bool,
}

fn show_menu(mut egui_ctx: ResMut<EguiContext>, mut sub_menu: ResMut<SubMenu>) {
    let view_model = match &mut *sub_menu {
        SubMenu::BrowseLobbies(BrowseLobbiesSubMenu::Main(view_model)) => view_model,
        _ => return,
    };
    if view_model.back {
        *sub_menu = SubMenu::Main;
        return;
    }
    if let Some(_lobby_name) = &view_model.join_lobby {
        *sub_menu = SubMenu::BrowseLobbies(BrowseLobbiesSubMenu::WaitingForPlayers(default()));
        return;
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        let center = ui.available_size() / 2.0;
        ui.allocate_ui_at_rect(
            egui::Rect::from_center_size(center.to_pos2(), egui::Vec2::new(500.0, 400.0)),
            |ui| {
                ui.push_id("Browsing Lobbies", |ui| {
                    ui.heading("Browsing Lobbies");
                });
                ui.add_space(100.0);
                ui.horizontal(|ui| {
                    ui.label("Player Name: ");
                    ui.text_edit_singleline(&mut view_model.player_name);
                    if !view_model.player_name.is_empty() {
                        view_model.player_name_empty_warning = false;
                    }
                    if view_model.player_name_empty_warning {
                        ui.add(egui::Label::new(
                            RichText::new("*").color(egui::Color32::RED),
                        ));
                    };
                });
                TableBuilder::new(ui)
                    .striped(true)
                    .column(Size::remainder().at_least(80.0))
                    .column(Size::initial(50.0))
                    .column(Size::initial(50.0))
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            ui.label("Choose a lobby");
                        });
                        header.col(|ui| {
                            ui.label("Players");
                        });
                    })
                    .body(|mut body| {
                        for _ in 0..100 {
                            body.row(30.0, |mut row| {
                                let lobby_name = "Lobby Name";
                                row.col(|ui| {
                                    ui.small(lobby_name);
                                });
                                row.col(|ui| {
                                    ui.label("0/8");
                                });
                                row.col(|ui| {
                                    if ui
                                        .add(egui::Button::new(egui::RichText::new("Join").small()))
                                        .clicked()
                                    {
                                        view_model.player_name_empty_warning =
                                            view_model.player_name.is_empty();
                                        if !view_model.player_name_empty_warning {
                                            view_model.join_lobby = Some(lobby_name.to_string());
                                        }
                                    };
                                });
                            });
                        }
                    });
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    if ui.button("Back").clicked() {
                        view_model.back = true;
                    }
                })
            },
        );
    });
}

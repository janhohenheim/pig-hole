use crate::GameState;

use self::create_lobby::CreateLobbyPlugin;
use self::main_menu::MainMenuPlugin;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, FontId},
    EguiContext,
};
use egui::FontFamily::*;
use egui::TextStyle;

mod create_lobby;
mod main_menu;
mod state;
use state::SubMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(configure_visuals);
        app.add_plugin(MainMenuPlugin).add_plugin(CreateLobbyPlugin);
        app.add_system_set(SystemSet::on_exit(GameState::Menu).with_system(reset_menu));
        app.init_resource::<SubMenu>();
    }
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_style(egui::Style {
        text_styles: [
            (TextStyle::Heading, FontId::new(30.0, Proportional)),
            (
                TextStyle::Name("Heading2".into()),
                FontId::new(25.0, Proportional),
            ),
            (
                TextStyle::Name("Context".into()),
                FontId::new(23.0, Proportional),
            ),
            (TextStyle::Body, FontId::new(18.0, Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, Proportional)),
            (TextStyle::Button, FontId::new(40.0, Proportional)),
            (TextStyle::Small, FontId::new(10.0, Proportional)),
        ]
        .into(),
        visuals: egui::Visuals {
            window_rounding: 0.0.into(),
            ..default()
        },
        ..default()
    });
}

fn reset_menu(mut sub_menu: ResMut<SubMenu>) {
    *sub_menu = default();
}

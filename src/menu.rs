use self::joining_lobby::JoiningLobbyPlugin;
use self::main_menu::MainMenuPlugin;
use crate::{
    loading::{FontAssets, MenuAssets},
    GameState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, FontId},
    EguiContext,
};
use egui::FontFamily::*;
use egui::TextStyle;

mod joining_lobby;
mod main_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(configure_visuals);
        app.add_plugin(MainMenuPlugin)
            .add_plugin(JoiningLobbyPlugin)
            .add_system_set(
                SystemSet::on_in_stack_update(GameState::Menu).with_system(hover_over_buttons),
            );
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

#[allow(clippy::type_complexity)]
fn hover_over_buttons(
    font_assets: Res<FontAssets>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<bevy::prelude::Button>),
    >,
) {
    let colors = MenuAssets::new(&font_assets)
        .button
        .colors
        .background
        .unwrap();
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {} // Handled by responsible menu plugin
            Interaction::Hovered => {
                *color = colors.hovered;
            }
            Interaction::None => {
                *color = colors.normal;
            }
        }
    }
}

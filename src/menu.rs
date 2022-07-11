use self::joining_lobby::JoiningLobbyPlugin;
use self::main_menu::MainMenuPlugin;
use crate::{
    loading::{FontAssets, MenuAssets},
    GameState,
};
use bevy::prelude::*;

mod joining_lobby;
mod main_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin)
            .add_plugin(JoiningLobbyPlugin)
            .add_system_set(
                SystemSet::on_in_stack_update(GameState::Menu).with_system(hover_over_buttons),
            );
    }
}

#[allow(clippy::type_complexity)]
fn hover_over_buttons(
    font_assets: Res<FontAssets>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let assets = MenuAssets::new(&font_assets).button;
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {} // Handled by responsible menu plugin
            Interaction::Hovered => {
                *color = assets.colors.hovered;
            }
            Interaction::None => {
                *color = assets.colors.normal;
            }
        }
    }
}

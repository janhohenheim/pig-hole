// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_game::GamePlugin;
#[cfg(feature = "dev")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        // Optimal power saving and present mode settings for desktop apps.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(WindowDescriptor {
            title: "Pig Hole".to_string(),
            ..default()
        })
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(GamePlugin).run();
}

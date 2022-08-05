use bevy::{
    app::ScheduleRunnerPlugin,
    core::CorePlugin,
    log::{self, LogPlugin},
    prelude::*,
};

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};


use shared::{channels::Channels, config::shared_config, protocol::Protocol};

mod resources;
mod systems;

use systems::{events, init::init, tick::tick};

fn main() {
    log::info!("Naia Bevy Server Demo starting up");

    // Build App
    App::default()
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .run();
}

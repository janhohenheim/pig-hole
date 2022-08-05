use bevy::{log, prelude::*};

use naia_bevy_client::Client;

use shared::{
    channels::Channels,
    protocol::{Auth, Protocol},
};

use crate::resources::Global;

pub fn init(mut commands: Commands, mut client: Client<Protocol, Channels>) {
    log::info!("Naia Bevy Client Demo started");

    client.auth(Auth::new("charlie", "12345"));
    log::info!("Authenticated");
    client.connect("http://127.0.0.1:14191");
    log::info!("Connected");

    // Setup Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Setup Colors
    commands.init_resource::<Global>();
}

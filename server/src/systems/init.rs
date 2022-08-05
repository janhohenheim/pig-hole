use std::collections::HashMap;

use bevy::log;
use bevy::prelude::*;

use naia_bevy_server::{Server, ServerAddrs};

use shared::{channels::Channels, protocol::Protocol};

use crate::resources::Global;

pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>) {
    log::info!("Naia Bevy Server Demo is running");

    // Naia Server initialization
    let server_addresses = ServerAddrs::new(
        "127.0.0.1:14191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "127.0.0.1:14192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://127.0.0.1:14192",
    );

    server.listen(&server_addresses);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();

    // Resources
    commands.insert_resource(Global {
        main_room_key,
        user_to_prediction_map: HashMap::new(),
    })
}

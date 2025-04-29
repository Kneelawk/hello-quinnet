use crate::client::ClientArgs;
use crate::common::{channels, C2SMsg, S2CMsg};
use bevy::app::{App, AppExit, Startup};
use bevy::prelude::*;
use bevy_quinnet::client::certificate::{CertificateVerificationMode, TrustOnFirstUseConfig};
use bevy_quinnet::client::connection::{
    ClientEndpointConfiguration, ConnectionEvent, ConnectionLostEvent, ConnectionState,
};
use bevy_quinnet::client::QuinnetClient;
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use std::net::Ipv4Addr;
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};

pub fn setup(app: &mut App) {
    app.add_systems(Startup, connect_to_server);
    app.add_systems(Update, disconnect_handler);
    app.add_systems(Update, connect_handler);
    app.add_systems(Update, message_handler);
    app.add_systems(Last, shutdown_disconnect);
}

pub fn connect_to_server(args: Res<ClientArgs>, mut client: ResMut<QuinnetClient>) {
    info!("Connecting to {} on port {}...", &args.server, args.port);

    client
        .open_connection(
            ClientEndpointConfiguration::from_addrs(
                (args.server.as_str(), args.port)
                    .to_socket_addrs()
                    .expect("The given hostname does not resolve")
                    .next()
                    .expect("The given hostname does not resolve"),
                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)),
            ),
            CertificateVerificationMode::TrustOnFirstUse(TrustOnFirstUseConfig::default()),
            ChannelsConfiguration::from_types(channels::CHANNELS.to_vec()).unwrap(),
        )
        .expect("Error connecting to server");
}

pub fn disconnect_handler(
    mut event: EventReader<ConnectionLostEvent>,
    mut shutdown: EventWriter<AppExit>,
) {
    for _ in event.read().take(1) {
        info!("Connection lost. Shutting down.");
        shutdown.write(AppExit::Success);
    }
}

pub fn shutdown_disconnect(mut shutdown: EventReader<AppExit>, mut client: ResMut<QuinnetClient>) {
    for _ in shutdown.read().take(1) {
        if client.connection().state() == ConnectionState::Connected {
            info!("Sending disconnect...");
            client
                .connection_mut()
                .send_message_on(channels::ORDERED_RELIABLE, C2SMsg::Disconnect)
                .ok();
        }
    }
}

pub fn connect_handler(mut events: EventReader<ConnectionEvent>, mut client: ResMut<QuinnetClient>) {
    for _ in events.read() {
        info!("Sending ping...");
        client
            .connection_mut()
            .send_message_on(channels::UNORDERED_RELIABLE, C2SMsg::Ping)
            .ok();
    }
}

pub fn message_handler(mut client: ResMut<QuinnetClient>, mut shutdown: EventWriter<AppExit>) {
    while let Ok(Some((_channel_id, msg))) = client.connection_mut().receive_message::<S2CMsg>() {
        match msg {
            S2CMsg::Pong => {
                info!("Received pong!");
            }
            S2CMsg::Disconnect => {
                info!("Server disconnect received. Shutting down...");
                shutdown.write(AppExit::Success);
            }
        }
    }
}

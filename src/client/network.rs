use crate::common::components::GameObject;
use crate::common::{C2SMsg, S2CMsg, channels};
use bevy::app::{App, AppExit};
use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClient;
use bevy_quinnet::client::certificate::{CertificateVerificationMode, TrustOnFirstUseConfig};
use bevy_quinnet::client::connection::{
    ClientEndpointConfiguration, ConnectionEvent, ConnectionLostEvent, ConnectionState,
};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use std::net::Ipv4Addr;
use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};

#[derive(Debug, Clone, Event)]
pub struct GameConnectionRequestedEvent {
    /// The ip or domain name of the server to connect to.
    server: String,

    /// The port of the server to connect to.
    port: u16,
}

#[derive(Default, Debug, Copy, Clone, Event)]
pub struct GameDisconnectedEvent;

#[derive(Default, Debug, Clone, PartialOrd, PartialEq, Resource)]
pub enum GameConnectionState {
    #[default]
    NotConnected,
    Connected {
        server: String,
        port: u16,
    },
}

pub fn setup(app: &mut App) {
    app.add_systems(FixedPostUpdate, connect_to_server);
    app.add_systems(FixedPreUpdate, disconnect_handler);
    app.add_systems(FixedPreUpdate, connect_handler);
    app.add_systems(FixedPreUpdate, message_handler);
    app.add_systems(FixedUpdate, remove_all_game_objects);
    app.add_systems(Last, shutdown_disconnect); // on main schedule to ensure run before game quits
    app.insert_resource(GameConnectionState::default());
}

pub fn remove_all_game_objects(
    mut event: EventReader<GameDisconnectedEvent>,
    game_objects: Query<Entity, With<GameObject>>,
    mut commands: Commands,
) {
    for _ in event.read().take(1) {
        info!("Deconstructing game state.");
        for e in game_objects {
            commands.entity(e).despawn();
        }
    }
}

pub fn connect_to_server(
    mut event: EventReader<GameConnectionRequestedEvent>,
    mut client: ResMut<QuinnetClient>,
    mut state: ResMut<GameConnectionState>,
) {
    for e in event.read() {
        if *state == GameConnectionState::NotConnected {
            info!("Connecting to {} on port {}...", &e.server, e.port);

            client
                .open_connection(
                    ClientEndpointConfiguration::from_addrs(
                        (e.server.as_str(), e.port)
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

            *state = GameConnectionState::Connected {
                server: e.server.clone(),
                port: e.port,
            };
        }
    }
}

pub fn disconnect_handler(
    mut event: EventReader<ConnectionLostEvent>,
    mut state: ResMut<GameConnectionState>,
    mut disconnect: EventWriter<GameDisconnectedEvent>,
) {
    for _ in event.read().take(1) {
        info!("Connection lost.");
        *state = GameConnectionState::NotConnected;
        disconnect.write(GameDisconnectedEvent);
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

pub fn connect_handler(
    mut events: EventReader<ConnectionEvent>,
    mut client: ResMut<QuinnetClient>,
) {
    for _ in events.read() {
        info!("Sending ping...");
        client
            .connection_mut()
            .send_message_on(channels::UNORDERED_RELIABLE, C2SMsg::Ping)
            .ok();
    }
}

pub fn message_handler(
    mut client: ResMut<QuinnetClient>,
    mut state: ResMut<GameConnectionState>,
    mut disconnect: EventWriter<GameDisconnectedEvent>,
) {
    while let Ok(Some((_channel_id, msg))) = client.connection_mut().receive_message::<S2CMsg>() {
        match msg {
            S2CMsg::Pong => {
                info!("Received pong!");
            }
            S2CMsg::Disconnect => {
                info!("Server disconnect received");
                *state = GameConnectionState::NotConnected;
                disconnect.write(GameDisconnectedEvent);
            }
        }
    }
}

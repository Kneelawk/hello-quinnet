use crate::common::{channels, C2SMsg, S2CMsg};
use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy_quinnet::client::certificate::{CertificateVerificationMode, TrustOnFirstUseConfig};
use bevy_quinnet::client::connection::{
    ClientEndpointConfiguration, ConnectionEvent, ConnectionLostEvent, ConnectionState,
};
use bevy_quinnet::client::{QuinnetClient, QuinnetClientPlugin};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use bevy_quinnet::shared::ClientId;
use bevy_rapier2d::prelude::*;
use clap::Args;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

#[derive(Debug, Default, Component)]
pub struct ClientPlayer;

#[derive(Debug, Default, Component)]
pub struct OtherPlayer {
    id: ClientId,
}

#[derive(Args, Resource, Debug)]
pub struct ClientArgs {
    /// The ip or domain name of the server to connect to.
    server: String,

    /// The port of the server to connect to.
    #[arg(short, long, default_value_t = 22223)]
    port: u16,
}

pub fn setup(app: &mut App, args: ClientArgs) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Hello Quinnet".to_string(),
            ..Default::default()
        }),
        exit_condition: ExitCondition::OnAllClosed,
        ..Default::default()
    }));
    app.add_plugins(QuinnetClientPlugin::default());
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_plugins(ClientPlugin);
    app.insert_resource(args);
}

struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, connect_to_server);
        app.add_systems(Update, disconnect_handler);
        app.add_systems(Update, connect_handler);
        app.add_systems(Update, message_handler);
        app.add_systems(Last, shutdown_disconnect);
    }
}

fn connect_to_server(args: Res<ClientArgs>, mut client: ResMut<QuinnetClient>) {
    info!("Connecting to {} on port {}...", &args.server, args.port);

    client
        .open_connection(
            ClientEndpointConfiguration::from_ips(
                IpAddr::from_str(&args.server).expect("Error parsing ip"),
                args.port,
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                0,
            ),
            CertificateVerificationMode::TrustOnFirstUse(TrustOnFirstUseConfig::default()),
            ChannelsConfiguration::from_types(channels::CHANNELS.to_vec()).unwrap(),
        )
        .expect("Error connecting to server");
}

fn disconnect_handler(
    mut event: EventReader<ConnectionLostEvent>,
    mut shutdown: EventWriter<AppExit>,
) {
    for _ in event.read().take(1) {
        info!("Connection lost. Shutting down.");
        shutdown.send(AppExit::Success);
    }
}

fn shutdown_disconnect(mut shutdown: EventReader<AppExit>, client: Res<QuinnetClient>) {
    for _ in shutdown.read().take(1) {
        if client.connection().state() == ConnectionState::Connected {
            info!("Sending disconnect...");
            client
                .connection()
                .send_message_on(channels::ORDERED_RELIABLE, C2SMsg::Disconnect)
                .ok();
        }
    }
}

fn connect_handler(mut events: EventReader<ConnectionEvent>, client: Res<QuinnetClient>) {
    for _ in events.read() {
        info!("Sending ping...");
        client
            .connection()
            .send_message_on(channels::UNORDERED_RELIABLE, C2SMsg::Ping)
            .ok();
    }
}

fn message_handler(mut client: ResMut<QuinnetClient>, mut shutdown: EventWriter<AppExit>) {
    while let Ok(Some((_channel_id, msg))) = client.connection_mut().receive_message::<S2CMsg>() {
        match msg {
            S2CMsg::Pong => {
                info!("Received pong!");
            }
            S2CMsg::Disconnect => {
                info!("Server disconnect received. Shutting down...");
                shutdown.send(AppExit::Success);
            }
        }
    }
}

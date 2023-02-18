use crate::common::{C2SMsg, Labels, S2CMsg, ShuttingDown};
use bevy::prelude::*;
use bevy::window::WindowClosed;
use bevy_quinnet::client::certificate::{CertificateVerificationMode, TrustOnFirstUseConfig};
use bevy_quinnet::client::connection::{
    ConnectionConfiguration, ConnectionEvent, ConnectionLostEvent,
};
use bevy_quinnet::client::{Client, QuinnetClientPlugin};
use bevy_quinnet::shared::channel::ChannelId;
use bevy_quinnet::shared::ClientId;
use bevy_rapier2d::prelude::*;
use clap::Args;

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
        window: WindowDescriptor {
            title: "Hello Quinnet".to_string(),
            ..Default::default()
        },
        exit_on_all_closed: false,
        ..Default::default()
    }));
    app.add_plugin(QuinnetClientPlugin::default());
    app.add_plugin(RapierDebugRenderPlugin::default());
    app.add_plugin(ClientPlugin);
    app.insert_resource(args);
}

struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(connect_to_server);
        app.add_system(disconnect_handler.label(Labels::ShutdownRequester));
        app.add_system(window_close_handler.label(Labels::ShutdownRequester));
        app.add_system(
            shutdown_disconnect
                .after(Labels::ShutdownRequester)
                .before(Labels::ShutdownHandler),
        );
        app.add_system(connect_handler);
        app.add_system(message_handler.label(Labels::ShutdownRequester));
    }
}

fn connect_to_server(args: Res<ClientArgs>, mut client: ResMut<Client>) {
    info!("Connecting to {} on port {}...", &args.server, args.port);

    client
        .open_connection(
            ConnectionConfiguration::new(args.server.clone(), args.port, "[::]".to_string(), 0),
            CertificateVerificationMode::TrustOnFirstUse(TrustOnFirstUseConfig::default()),
        )
        .expect("Error connecting to server");
}

fn disconnect_handler(
    mut event: EventReader<ConnectionLostEvent>,
    mut shutdown: EventWriter<ShuttingDown>,
) {
    for _ in event.iter().take(1) {
        info!("Connection lost. Shutting down.");
        shutdown.send(ShuttingDown);
    }
}

fn window_close_handler(
    mut closed: EventReader<WindowClosed>,
    windows: Res<Windows>,
    mut shutdown: EventWriter<ShuttingDown>,
) {
    for _ in closed.iter().take(1) {
        if windows.iter().next().is_none() {
            shutdown.send(ShuttingDown);
        }
    }
}

fn shutdown_disconnect(mut shutdown: EventReader<ShuttingDown>, client: Res<Client>) {
    for _ in shutdown.iter().take(1) {
        if client.connection().is_connected() {
            client
                .connection()
                .send_message_on(ChannelId::UnorderedReliable, C2SMsg::Disconnect)
                .ok();
        }
    }
}

fn connect_handler(mut events: EventReader<ConnectionEvent>, client: Res<Client>) {
    for _ in events.iter() {
        client
            .connection()
            .send_message_on(ChannelId::UnorderedReliable, C2SMsg::Ping)
            .ok();
    }
}

fn message_handler(mut client: ResMut<Client>, mut shutdown: EventWriter<ShuttingDown>) {
    while let Ok(Some(msg)) = client.connection_mut().receive_message::<S2CMsg>() {
        match msg {
            S2CMsg::Pong => {
                info!("Received pong!");
            }
            S2CMsg::Disconnect => {
                info!("Server disconnect received. Shutting down...");
                shutdown.send(ShuttingDown);
            }
        }
    }
}

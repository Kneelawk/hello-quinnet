use crate::common::{C2SMsg, S2CMsg, channels};
use bevy::app::ScheduleRunnerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent, QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration, ServerMessageReceiveError, ServerReceiveError};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use clap::Args;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

#[derive(Args, Resource, Debug)]
pub struct ServerArgs {
    /// The port that this server will listen on.
    #[arg(short, long, default_value_t = 22223)]
    port: u16,
}

pub fn setup(app: &mut App, args: ServerArgs) {
    app.add_plugins(LogPlugin::default());
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
    );
    app.add_plugins(QuinnetServerPlugin::default());
    app.add_plugins(ServerPlugin);
    app.insert_resource(args);
}

struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_server);
        app.add_systems(Update, connect_handler);
        app.add_systems(Update, disconnect_handler);
        app.add_systems(Update, message_handler);
        app.add_systems(Last, shutdown_disconnect);
    }
}

fn start_server(args: Res<ServerArgs>, mut server: ResMut<QuinnetServer>) {
    info!("Starting server on port {}...", args.port);

    server
        .start_endpoint(
            ServerEndpointConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), args.port),
            CertificateRetrievalMode::LoadFromFileOrGenerateSelfSigned {
                cert_file: "server_certificates.pem".to_string(),
                key_file: "server_privkey.pem".to_string(),
                save_on_disk: true,
                server_hostname: "game.kneelawk.com".to_string(),
            },
            ChannelsConfiguration::from_types(channels::CHANNELS.to_vec()).unwrap(),
        )
        .expect("Error starting server");
}

fn shutdown_disconnect(mut exit: EventReader<AppExit>, mut server: ResMut<QuinnetServer>) {
    for _ in exit.read().take(1) {
        let endpoint = server.endpoint_mut();
        info!("Disconnecting all clients...");
        endpoint.try_send_group_message_on(
            endpoint.clients().iter(),
            channels::ORDERED_RELIABLE,
            S2CMsg::Disconnect,
        );
        endpoint.disconnect_all_clients();
    }
}

fn connect_handler(mut events: EventReader<ConnectionEvent>) {
    for event in events.read() {
        info!("Client connected: {}", event.id);
    }
}

fn disconnect_handler(mut events: EventReader<ConnectionLostEvent>) {
    for event in events.read() {
        info!("Client disconnected: {}", event.id);
    }
}

fn message_handler(mut server: ResMut<QuinnetServer>) {
    let endpoint = server.endpoint_mut();
    for client in endpoint.clients() {
        loop {
            let msg = endpoint.receive_message_from::<C2SMsg>(client);
            if msg.is_err() {
                if let Err(ServerMessageReceiveError::ReceiveError(ServerReceiveError::ConnectionClosed)) = msg {
                    info!("Client disconnected: {}", client);
                }
                break;
            }

            let msg = msg.unwrap();
            if msg.is_none() {
                break;
            }

            let (_channel_id, msg) = msg.unwrap();
            match msg {
                C2SMsg::Ping => {
                    info!("Received ping!");
                    info!("Sending pong...");
                    endpoint
                        .send_message_on(client, channels::UNORDERED_RELIABLE, S2CMsg::Pong)
                        .ok();
                }
                C2SMsg::Disconnect => {
                    info!("Client disconnect received: {}", client);
                    endpoint.disconnect_client(client).ok();
                }
            }
        }
    }
}

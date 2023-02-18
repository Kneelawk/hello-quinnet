use crate::common::{C2SMsg, S2CMsg, ShuttingDown};
use bevy::app::ScheduleRunnerSettings;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{
    ConnectionEvent, ConnectionLostEvent, QuinnetServerPlugin, Server, ServerConfigurationData,
};
use bevy_quinnet::shared::channel::ChannelId;
use clap::Args;
use std::time::Duration;

#[derive(Args, Resource, Debug)]
pub struct ServerArgs {
    /// The port that this server will listen on.
    #[arg(short, long, default_value_t = 22223)]
    port: u16,
}

pub fn setup(app: &mut App, args: ServerArgs) {
    app.add_plugin(LogPlugin::default());
    app.add_plugins(MinimalPlugins);
    app.add_plugin(QuinnetServerPlugin::default());
    app.add_plugin(ServerPlugin);
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )));
    app.insert_resource(args);
}

struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_server);
        app.add_system(shutdown_disconnect);
        app.add_system(connect_handler);
        app.add_system(disconnect_handler);
        app.add_system(message_handler);
    }
}

fn start_server(args: Res<ServerArgs>, mut server: ResMut<Server>) {
    info!("Starting server on port {}...", args.port);

    server
        .start_endpoint(
            ServerConfigurationData::new("127.0.0.1".to_string(), args.port, "[::]".to_string()),
            CertificateRetrievalMode::LoadFromFileOrGenerateSelfSigned {
                cert_file: "server_certificates.pem".to_string(),
                key_file: "server_privkey.pem".to_string(),
                save_on_disk: true,
            },
        )
        .expect("Error starting server");
}

fn shutdown_disconnect(mut exit: EventReader<ShuttingDown>, mut server: ResMut<Server>) {
    for _ in exit.iter().take(1) {
        let endpoint = server.endpoint_mut();
        endpoint.try_send_group_message_on(
            endpoint.clients().iter(),
            ChannelId::UnorderedReliable,
            S2CMsg::Disconnect,
        );
        endpoint.disconnect_all_clients().ok();
    }
}

fn connect_handler(mut events: EventReader<ConnectionEvent>) {
    for event in events.iter() {
        info!("Client connected: {}", event.id);
    }
}

fn disconnect_handler(mut events: EventReader<ConnectionLostEvent>) {
    for event in events.iter() {
        info!("Client disconnected: {}", event.id);
    }
}

fn message_handler(mut server: ResMut<Server>) {
    let endpoint = server.endpoint_mut();
    for client in endpoint.clients() {
        while let Some(msg) = endpoint.try_receive_message_from::<C2SMsg>(client) {
            match msg {
                C2SMsg::Ping => {
                    info!("Received ping!");
                    endpoint
                        .send_message_on(client, ChannelId::UnorderedReliable, S2CMsg::Pong)
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

mod network;
mod player;

use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy_quinnet::client::QuinnetClientPlugin;
use clap::Args;

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
    // app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_plugins(ClientPlugin);
    app.insert_resource(args);
}

struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        network::setup(app);
    }
}

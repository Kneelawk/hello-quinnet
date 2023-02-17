use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_quinnet::client::QuinnetClientPlugin;
use clap::Args;

#[derive(Args, Resource, Debug)]
pub struct ClientArgs {}

pub fn setup(app: &mut App, args: ClientArgs) {
    app.add_plugins(DefaultPlugins);
    app.add_plugin(QuinnetClientPlugin::default());
    app.add_plugin(RapierDebugRenderPlugin::default());
    app.insert_resource(args);
    app.add_startup_system(startup);
}

fn startup() {
    info!("Starting client...");
}

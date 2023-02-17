use bevy::app::App;
use bevy::prelude::Resource;
use bevy::DefaultPlugins;
use bevy_quinnet::client::QuinnetClientPlugin;
use clap::Args;

#[derive(Args, Resource, Debug)]
pub struct ClientArgs {}

pub fn add_plugins(app: &mut App, args: ClientArgs) {
    app.add_plugins(DefaultPlugins);
    app.add_plugin(QuinnetClientPlugin::default());
    app.insert_resource(args);
    app.add_startup_system(startup);
}

fn startup() {
    info!("Starting client...");
}

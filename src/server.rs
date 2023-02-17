use bevy::app::App;
use bevy::log::LogPlugin;
use bevy::prelude::Resource;
use bevy::MinimalPlugins;
use bevy_quinnet::server::QuinnetServerPlugin;
use clap::Args;

#[derive(Args, Resource, Debug)]
pub struct ServerArgs {}

pub fn add_plugins(app: &mut App, args: ServerArgs) {
    app.add_plugin(LogPlugin::default());
    app.add_plugins(MinimalPlugins);
    app.add_plugin(QuinnetServerPlugin::default());
    app.insert_resource(args);
    app.add_startup_system(startup);
}

fn startup() {
    info!("Starting server...");
}

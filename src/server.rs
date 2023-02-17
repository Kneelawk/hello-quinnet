use bevy::app::ScheduleRunnerSettings;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServerPlugin;
use clap::Args;
use std::time::Duration;

#[derive(Args, Resource, Debug)]
pub struct ServerArgs {}

pub fn setup(app: &mut App, args: ServerArgs) {
    app.add_plugin(LogPlugin::default());
    app.add_plugins(MinimalPlugins);
    app.add_plugin(QuinnetServerPlugin::default());
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )));
    app.insert_resource(args);
    app.add_startup_system(startup);
}

fn startup() {
    info!("Starting server...");
}

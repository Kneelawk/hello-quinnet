mod components;
mod events;
mod labels;
mod messages;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub use components::*;
pub use events::*;
pub use labels::*;
pub use messages::*;

static CTRL_C: AtomicBool = AtomicBool::new(false);

pub fn setup(app: &mut App) {
    ctrlc::set_handler(move || {
        CTRL_C.store(true, Ordering::Release);
    })
    .expect("Error setting Ctrl-C handler");

    // Physics
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0));
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..Default::default()
    });

    // Common stuff
    app.add_event::<ShuttingDown>();
    app.add_system(ctrlc_handler.label(Labels::ShutdownRequester));
    app.add_system(shutdown_handler.label(Labels::ShutdownHandler));
}

fn ctrlc_handler(mut event: EventWriter<ShuttingDown>) {
    if CTRL_C.load(Ordering::Acquire) {
        event.send(ShuttingDown);
    }
}

fn shutdown_handler(mut shutdown: EventReader<ShuttingDown>, mut exit: EventWriter<AppExit>) {
    for _ in shutdown.iter().take(1) {
        info!("Shutting down...");
        exit.send(AppExit);
    }
}

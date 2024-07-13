pub mod channels;
mod components;
mod events;
mod messages;
mod shutdown;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub use messages::*;
pub use shutdown::*;

static CTRL_C: AtomicBool = AtomicBool::new(false);

pub fn setup(app: &mut App) {
    ctrlc::set_handler(move || {
        CTRL_C.store(true, Ordering::Release);
    })
    .expect("Error setting Ctrl-C handler");

    // Physics
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0));
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..RapierConfiguration::new(16.0)
    });

    // Common stuff
    app.add_event::<RequestShutdown>();
    app.add_event::<ShuttingDown>();
    app.add_event::<PostShutdown>();
    app.insert_resource(ShutdownState::None);
    app.add_systems(PreUpdate, ctrlc_handler);
    app.add_systems(Last, shutdown_handler);
}

fn ctrlc_handler(mut event: EventWriter<RequestShutdown>) {
    if CTRL_C.load(Ordering::Acquire) {
        event.send(RequestShutdown);
    }
}

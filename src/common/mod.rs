pub mod channels;
mod components;
mod events;
mod messages;

use bevy::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub use messages::*;

static CTRL_C: AtomicBool = AtomicBool::new(false);

pub fn setup(app: &mut App) {
    ctrlc::set_handler(move || {
        CTRL_C.store(true, Ordering::Release);
    })
    .expect("Error setting Ctrl-C handler");

    // Physics
    // app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0));
    // app.insert_resource(RapierConfiguration {
    //     gravity: Vec2::ZERO,
    //     ..RapierConfiguration::new(16.0)
    // });

    // Common stuff
    app.add_systems(PostUpdate, ctrlc_handler);
}

fn ctrlc_handler(mut event: EventWriter<AppExit>) {
    if CTRL_C.load(Ordering::Acquire) {
        event.write(AppExit::Success);
    }
}

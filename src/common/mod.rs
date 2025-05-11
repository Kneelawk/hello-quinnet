pub mod channels;
pub mod components;
pub mod events;
mod messages;

use bevy::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use avian2d::PhysicsPlugins;
pub use messages::*;

/// The default port that servers are hosted on
pub const DEFAULT_PORT: u16 = 22223;

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
    app.add_plugins(PhysicsPlugins::default().with_length_unit(8.0));
    app.insert_resource(SceneSpawner::default());

    // Common stuff
    app.add_systems(PostUpdate, ctrlc_handler);
}


fn ctrlc_handler(mut event: EventWriter<AppExit>) {
    if CTRL_C.load(Ordering::Acquire) {
        event.write(AppExit::Success);
    }
}

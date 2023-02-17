use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0));
}

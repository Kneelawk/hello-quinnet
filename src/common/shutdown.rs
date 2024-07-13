use bevy::app::AppExit;
use bevy::prelude::{Commands, Event, EventReader, EventWriter, ResMut, Resource, World};

#[derive(Event)]
pub struct RequestShutdown;

#[derive(Event)]
pub struct ShuttingDown;

#[derive(Event)]
pub struct PostShutdown;

#[derive(Resource, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ShutdownState {
    None,
    ShuttingDown,
}

pub fn shutdown_handler(
    mut state: ResMut<ShutdownState>,
    mut request_read: EventReader<RequestShutdown>,
    mut shutdown_read: EventReader<ShuttingDown>,
    mut post_read: EventReader<PostShutdown>,
    mut exit: EventWriter<AppExit>,
    mut commands: Commands,
) {
    // handle starting the shutdown
    for _ in request_read.read().take(1) {
        if *state != ShutdownState::ShuttingDown {
            info!("Sending shutdown signal...");
            *state = ShutdownState::ShuttingDown;
            commands.add(|w: &mut World| {
                w.send_event(ShuttingDown);
            });
        }
    }

    // handle ending the shutdown
    for _ in shutdown_read.read().take(1) {
        info!("Ending shutdown phase...");
        commands.add(|w: &mut World| {
            w.send_event(PostShutdown);
        });
    }

    // handle stopping the application
    for _ in post_read.read().take(1) {
        info!("Exiting app...");
        exit.send(AppExit::Success);
    }
}

use bevy::prelude::SystemLabel;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, SystemLabel)]
pub enum Labels {
    ShutdownRequester,
    ShutdownHandler,
}

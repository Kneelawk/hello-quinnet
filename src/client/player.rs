use bevy::prelude::Component;
use bevy_quinnet::shared::ClientId;

#[derive(Debug, Default, Component, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClientPlayer;

#[derive(Debug, Default, Component, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OtherPlayer {
    id: ClientId,
}

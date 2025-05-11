use bevy::prelude::Component;

/// Marks a player entity as the one controlled by this client.
#[derive(Debug, Default, Component, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClientPlayer;

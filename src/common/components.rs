use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};

/// Marks a player-controlled entity with the player that is controlling it
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component, Serialize, Deserialize)]
pub struct Player {
    pub id: ClientId,
}

/// Marks an entity that should be despawned when a game shuts down or a client is disconnected
#[derive(Debug, Default, Copy, Clone, Component, Serialize, Deserialize)]
pub struct GameObject;

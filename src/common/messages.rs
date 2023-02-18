use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum C2SMsg {
    Ping,
    Disconnect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum S2CMsg {
    Pong,
    Disconnect,
}

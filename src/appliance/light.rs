use super::common::Button;
use serde::{Deserialize, Serialize};

/// Light
#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
    pub state: LightState,
    pub buttons: Vec<Button>,
}

/// LightState
#[derive(Serialize, Deserialize, Debug)]
pub struct LightState {
    pub brightness: String,
    pub power: String,
    pub last_button: String,
}

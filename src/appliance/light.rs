use super::common::Button;
use serde::{Deserialize, Serialize};

/// Light
#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
    state: LightState,
    buttons: Vec<Button>,
}

/// LightState
#[derive(Serialize, Deserialize, Debug)]
pub struct LightState {
    brightness: String,
    power: String,
    last_button: String,
}

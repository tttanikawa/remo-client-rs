use super::Button;
use serde::{Deserialize, Serialize};

/// TV
#[derive(Serialize, Deserialize, Debug)]
pub struct TV {
    state: TVState,
    buttons: Vec<Button>,
}

/// TVState
#[derive(Serialize, Deserialize, Debug)]
pub struct TVState {
    input: String,
}

use super::Button;
use serde::{Deserialize, Serialize};

/// TV
#[derive(Serialize, Deserialize, Debug)]
pub struct TV {
    pub state: TVState,
    pub buttons: Vec<Button>,
}

/// TVState
#[derive(Serialize, Deserialize, Debug)]
pub struct TVState {
    pub input: String,
}

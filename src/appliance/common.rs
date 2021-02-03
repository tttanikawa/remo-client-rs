use serde::{Deserialize, Serialize};

/// Signal
#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    id: String,
    name: String,
    image: String,
}

/// Button
#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    name: String,
    image: String,
    label: String,
}

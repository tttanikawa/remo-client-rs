use serde::{Deserialize, Serialize};

/// Signal
#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    pub id: String,
    pub name: String,
    pub image: String,
}

/// Button
#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    pub name: String,
    pub image: String,
    pub label: String,
}

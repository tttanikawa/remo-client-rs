use serde::{Deserialize, Serialize};

/// SmartMeter
#[derive(Serialize, Deserialize, Debug)]
pub struct SmartMeter {
    pub echonetlite_properties: Vec<EchonetLiteProperty>,
    pub power: String,
    pub last_button: String,
}

/// EchonetLiteProperty
#[derive(Serialize, Deserialize, Debug)]
pub struct EchonetLiteProperty {
    pub name: String,
    pub epic: i64,
    pub val: String,
    pub updated_at: String,
}

use serde::{Deserialize, Serialize};

/// SmartMeter
#[derive(Serialize, Deserialize, Debug)]
pub struct SmartMeter {
    echonetlite_properties: Vec<EchonetLiteProperty>,
    power: String,
    last_button: String,
}

/// EchonetLiteProperty
#[derive(Serialize, Deserialize, Debug)]
pub struct EchonetLiteProperty {
    name: String,
    epic: i64,
    val: String,
    updated_at: String,
}

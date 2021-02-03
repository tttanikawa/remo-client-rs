use serde::{Deserialize, Serialize};

/// AirConParams
#[derive(Serialize, Deserialize, Debug)]
pub struct AirConParams {
    temp: String,
    mode: String,
    vol: String,
    dir: String,
    button: String,
}

/// AirConParams
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AirCon {
    range: Range,
    temp_unit: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    modes: Modes,
    fixed_buttons: Vec<String>,
}

/// Modes
#[derive(Serialize, Deserialize, Debug)]
pub struct Modes {
    cool: Option<AirConRangeMode>,
    warm: Option<AirConRangeMode>,
    dry: Option<AirConRangeMode>,
    blow: Option<AirConRangeMode>,
    auto: Option<AirConRangeMode>,
}

/// AirConRangeMode
#[derive(Serialize, Deserialize, Debug)]
pub struct AirConRangeMode {
    temp: Vec<String>,
    vol: Vec<String>,
    dir: Vec<String>,
    dirh: Vec<String>,
}

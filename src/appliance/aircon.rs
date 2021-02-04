use crate::{appliance::ApplianceModel, client::Client};
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
    pub temp: Vec<String>,
    pub vol: Vec<String>,
    pub dir: Vec<String>,
    pub dirh: Vec<String>,
}

/// InfraredMessage
#[derive(Serialize, Deserialize, Debug)]
pub struct InfraredMessage {
    pub freq: u64,
    pub data: Vec<u64>,
    pub format: String,
}

/// ApplianceModelAndParams
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplianceModelAndParam {
    pub model: ApplianceModel,
    pub params: AirConParams,
}

impl Client {
    /// Find the air conditioner best matching the provided infrared signal
    pub async fn detect_appliance(
        &self,
        message: &InfraredMessage,
    ) -> Result<ApplianceModelAndParam, Box<dyn std::error::Error>> {
        let message = serde_json::to_string(message)?;
        let response = self
            .request(
                reqwest::Method::POST,
                "/1/detectappliance",
                &maplit::btreemap! {"message" => message.as_str()},
            )
            .await?
            .text()
            .await?;
        let response: ApplianceModelAndParam = serde_json::from_str(&response).unwrap();
        Ok(response)
    }
}

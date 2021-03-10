use crate::{appliance::ApplianceModel, client::Client};
use serde::{Deserialize, Serialize};

/// AirConParams
#[derive(Serialize, Deserialize, Debug)]
pub struct AirConParams {
    pub temp: Option<String>,
    pub mode: Option<String>,
    pub vol: Option<String>,
    pub dir: Option<String>,
    pub button: Option<String>,
}

/// AirConParams
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AirCon {
    pub range: Range,
    pub temp_unit: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub modes: Modes,
    pub fixed_buttons: Vec<String>,
}

/// Modes
#[derive(Serialize, Deserialize, Debug)]
pub struct Modes {
    pub cool: Option<AirConRangeMode>,
    pub warm: Option<AirConRangeMode>,
    pub dry: Option<AirConRangeMode>,
    pub blow: Option<AirConRangeMode>,
    pub auto: Option<AirConRangeMode>,
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

    /// Update air conditioner settings
    pub async fn update_air_con_settings(
        &self,
        appliance_id: &str,
        ac_params: &AirConParams,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let params = {
            let mut params = maplit::btreemap! {};
            if let Some(temp) = &ac_params.temp {
                params.insert("temperature", temp.as_str());
            }
            if let Some(mode) = &ac_params.mode {
                params.insert("operation_mode", mode.as_str());
            }
            if let Some(vol) = &ac_params.vol {
                params.insert("air_volume", vol.as_str());
            }
            if let Some(dir) = &ac_params.dir {
                params.insert("air_direction", dir.as_str());
            }
            if let Some(button) = &ac_params.button {
                params.insert("button", button.as_str());
            }
            params
        };
        let status = self
            .request(
                reqwest::Method::POST,
                &format!("/1/appliances/{}/aircon_settings", appliance_id),
                &params,
            )
            .await?
            .error_for_status();
        match status {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}

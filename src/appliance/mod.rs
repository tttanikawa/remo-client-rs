mod aircon;
mod common;
mod light;
mod smart_meter;
mod tv;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::device::DeviceCore;
pub use aircon::*;
pub use common::*;
pub use light::*;
pub use smart_meter::*;
pub use tv::*;

/// Appliance
#[derive(Serialize, Deserialize, Debug)]
pub struct Appliance {
    id: String,
    device: DeviceCore,
    model: Option<ApplianceModel>,
    nickname: String,
    image: String,
    #[serde(rename = "type")]
    app_type: String,
    settings: Option<AirConParams>,
    aircon: Option<AirCon>,
    signals: Vec<Signal>,
    tv: Option<TV>,
    light: Option<Light>,
    smart_meter: Option<SmartMeter>,
}

/// ApplianceModel
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplianceModel {
    id: String,
    manufacturer: String,
    remote_name: String,
    name: String,
    image: String,
    country: String,
    series: Option<String>,
}

impl Client {
    pub async fn get_appliances(&self) -> Result<Vec<Appliance>, reqwest::Error> {
        let response = self
            .request(reqwest::Method::GET, "/1/appliances")
            .await?
            .text()
            .await?;
        let devices: Vec<Appliance> = match serde_json::from_str(&response) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{:?}", e);
                vec![]
            }
        };
        Ok(devices)
    }

    pub async fn get_appliance_signals(
        &self,
        appliance_id: &str,
    ) -> Result<Vec<Signal>, reqwest::Error> {
        let response = self
            .request(
                reqwest::Method::GET,
                &format!("/1/appliances/{}/signals", appliance_id),
            )
            .await?
            .text()
            .await?;
        let signals: Vec<Signal> = match serde_json::from_str(&response) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{:?}", e);
                vec![]
            }
        };
        Ok(signals)
    }
}

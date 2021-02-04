mod aircon;
mod common;
mod light;
mod smart_meter;
mod tv;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    pub id: String,
    pub device: DeviceCore,
    pub model: Option<ApplianceModel>,
    pub nickname: String,
    pub image: String,
    #[serde(rename = "type")]
    pub app_type: String,
    pub settings: Option<AirConParams>,
    pub aircon: Option<AirCon>,
    pub signals: Vec<Signal>,
    pub tv: Option<TV>,
    pub light: Option<Light>,
    pub smart_meter: Option<SmartMeter>,
}

/// ApplianceModel
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplianceModel {
    pub id: String,
    pub manufacturer: String,
    pub remote_name: String,
    pub name: String,
    pub image: String,
    pub country: String,
    pub series: Option<String>,
}

impl Client {
    pub async fn get_appliances(&self) -> Result<Vec<Appliance>, reqwest::Error> {
        let response = self
            .request(reqwest::Method::GET, "/1/appliances", &BTreeMap::new())
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
                &BTreeMap::new(),
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

    /// Create a new appliance
    pub async fn create_appliance(
        &self,
        nickname: &str,
        model: Option<&str>,
        model_type: Option<&str>,
        device: &str,
        image: &str,
    ) -> Result<Appliance, Box<dyn std::error::Error>> {
        let mut params = maplit::btreemap! {
            "nickname"=> nickname,
            "device" => device,
            "image" => image
        };
        if let Some(model) = model {
            params.insert("model", model);
        }
        if let Some(model_type) = model_type {
            params.insert("model_type", model_type);
        }

        let response = self
            .request(reqwest::Method::POST, "/1/appliances", &params)
            .await?
            .text()
            .await?;
        let appliance: Appliance = serde_json::from_str(&response).unwrap();
        Ok(appliance)
    }
}

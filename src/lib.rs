use reqwest::header;
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::collections::BTreeMap;

/// User information
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    nickname: String,
    superuser: Option<bool>,
}

/// Devices information
#[derive(Serialize, Deserialize, Debug)]
pub struct SensorValue {
    val: f64,
    created_at: String,
}

/// Devices information
#[derive(Serialize, Deserialize, Debug)]
pub struct Events {
    te: SensorValue,
    hu: SensorValue,
    il: SensorValue,
    mo: SensorValue,
}

/// Devices information
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    id: String,
    name: String,
    temperature_offset: i64,
    humidity_offset: i64,
    created_at: String,
    updated_at: String,
    firmware_version: String,
    mac_address: String,
    bt_mac_address: String,
    serial_number: String,
    newest_events: Events,
    users: Vec<User>,
}

/// DevicesCore
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceCore {
    id: String,
    name: String,
    temperature_offset: i64,
    humidity_offset: i64,
    created_at: String,
    updated_at: String,
    firmware_version: String,
    mac_address: String,
    bt_mac_address: String,
    serial_number: String,
}

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

/// Signal
#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    id: String,
    name: String,
    image: String,
}

/// TV
#[derive(Serialize, Deserialize, Debug)]
pub struct TV {
    state: TVState,
    buttons: Vec<Button>,
}

/// TVState
#[derive(Serialize, Deserialize, Debug)]
pub struct TVState {
    input: String,
}

/// Button
#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    name: String,
    image: String,
    label: String,
}

/// Light
#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
    state: LightState,
    buttons: Vec<Button>,
}

/// LightState
#[derive(Serialize, Deserialize, Debug)]
pub struct LightState {
    brightness: String,
    power: String,
    last_button: String,
}

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

pub struct Client {
    access_token: String,
    base_url: &'static str,
}

impl Client {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            base_url: "https://api.nature.global/1",
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        url: &str,
        params: &BTreeMap<&str, &str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let header_map = {
            let mut map = header::HeaderMap::new();
            map.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", self.access_token).parse().unwrap(),
            );
            map.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("application/json"),
            );
            map
        };
        let client = reqwest::Client::new();
        client
            .request(method, &format!("{}{}", self.base_url, url))
            .headers(header_map)
            .send()
            .await
    }

    pub async fn get_user(&self) -> Result<User, reqwest::Error> {
        let response = self
            .request(reqwest::Method::GET, "/users/me", &BTreeMap::new())
            .await?
            .text()
            .await?;
        let user: User = serde_json::from_str(&response).unwrap();
        Ok(user)
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>, reqwest::Error> {
        let response = self
            .request(reqwest::Method::GET, "/devices", &BTreeMap::new())
            .await?
            .text()
            .await?;
        let devices: Vec<Device> = serde_json::from_str(&response).unwrap();
        Ok(devices)
    }

    pub async fn get_appliances(&self) -> Result<Vec<Appliance>, reqwest::Error> {
        let response = self
            .request(reqwest::Method::GET, "/appliances", &BTreeMap::new())
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
                &format!("/appliances/{}/signals", appliance_id),
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
}

#[cfg(test)]
mod test_remo_client_rs {
    use super::*;
    use std::fs;

    #[test]
    fn test_user_deserialize() {
        let json = fs::read_to_string("samples/user.json").unwrap();
        let user: User = serde_json::from_str(&json).unwrap();

        assert_eq!(user.id, "test_user_0");
        assert_eq!(user.nickname, "Test user");
    }

    #[test]
    fn test_devices_deserialize() {
        let json = fs::read_to_string("samples/devices.json").unwrap();
        let device: Vec<Device> = serde_json::from_str(&json).unwrap();

        assert_eq!(device[0].id, "my_nature_remo_3");
        assert_eq!(device[0].name, "  Remo 3       ");
        assert_eq!(device[0].temperature_offset, 0);
        assert_eq!(device[0].humidity_offset, 0);
        assert_eq!(device[0].created_at, "2020-12-20T03:06:40Z");
        assert_eq!(device[0].updated_at, "2021-01-31T15:37:25Z");
        assert_eq!(device[0].firmware_version, "Remo/1.2.14");
        assert_eq!(device[0].mac_address, "a1:b2:c3:d4:e5:f6");
        assert_eq!(device[0].bt_mac_address, "a1:b2:c3:d4:e5:f7");
        assert_eq!(device[0].serial_number, "1W0123456789");
        assert_eq!(device[0].newest_events.hu.val, 43.0);
        assert_eq!(
            device[0].newest_events.hu.created_at,
            "2021-01-31T16:46:08Z"
        );
        assert_eq!(device[0].newest_events.il.val, 5.0);
        assert_eq!(
            device[0].newest_events.il.created_at,
            "2021-01-31T16:46:28Z"
        );
        assert_eq!(device[0].newest_events.mo.val, 1.0);
        assert_eq!(
            device[0].newest_events.mo.created_at,
            "2021-01-31T16:06:08Z"
        );
        assert_eq!(device[0].newest_events.te.val, 19.1);
        assert_eq!(
            device[0].newest_events.te.created_at,
            "2021-01-31T16:58:09Z"
        );
    }
}

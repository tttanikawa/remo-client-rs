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

pub struct Client {
    access_token: String,
}

impl Client {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
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
        client.request(method, url).headers(header_map).send().await
    }

    pub async fn get_user(&self) -> Result<User, reqwest::Error> {
        let response = self
            .request(
                reqwest::Method::GET,
                "https://api.nature.global/1/users/me",
                &BTreeMap::new(),
            )
            .await?
            .text()
            .await?;
        let user: User = serde_json::from_str(response.as_str()).unwrap();
        Ok(user)
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>, reqwest::Error> {
        let response = self
            .request(
                reqwest::Method::GET,
                "https://api.nature.global/1/devices",
                &BTreeMap::new(),
            )
            .await?
            .text()
            .await?;
        let devices: Vec<Device> = serde_json::from_str(response.as_str()).unwrap();
        Ok(devices)
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

        assert_eq!(user.id, "0123456789abcdefg");
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

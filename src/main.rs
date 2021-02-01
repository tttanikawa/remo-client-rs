use dotenv::dotenv;
use reqwest::header;
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::{collections::BTreeMap, env};

/// User information
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    nickname: String,
    superuser: Option<bool>,
}

/// Devices information
#[derive(Serialize, Deserialize, Debug)]
struct SensorValue {
    val: f64,
    created_at: String,
}

/// Devices information
#[derive(Serialize, Deserialize, Debug)]
struct Events {
    te: SensorValue,
    hu: SensorValue,
    il: SensorValue,
    mo: SensorValue,
}

/// Devices information
#[derive(Serialize, Deserialize, Debug)]
struct Device {
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

struct Client {
    access_token: String,
}

impl Client {
    fn new(access_token: impl Into<String>) -> Self {
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

    async fn get_user(&self) -> Result<User, reqwest::Error> {
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

    async fn get_devices(&self) -> Result<Vec<Device>, reqwest::Error> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token =
        env::var("NATURE_REMO_ACCESS_TOKEN").expect("NATURE_REMO_ACCESS_TOKEN is not defined");
    let client = Client::new(token);

    let user = client.get_user().await?;
    println!("{:?}", user);

    let devices = client.get_devices().await?;
    println!("{:?}", devices);

    Ok(())
}

use crate::client::Client;
use crate::user::User;
use serde::{Deserialize, Serialize};
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

/// SensorValue
#[derive(Serialize, Deserialize, Debug)]
pub struct SensorValue {
    val: f64,
    created_at: String,
}

/// Events
#[derive(Serialize, Deserialize, Debug)]
pub struct Events {
    te: SensorValue,
    hu: SensorValue,
    il: SensorValue,
    mo: SensorValue,
}

impl Client {
    pub async fn get_devices(&self) -> Result<Vec<Device>, reqwest::Error> {
        let response = self
            .request(reqwest::Method::GET, "/1/devices")
            .await?
            .text()
            .await?;
        let devices: Vec<Device> = serde_json::from_str(&response).unwrap();
        Ok(devices)
    }
}

#[cfg(test)]
mod test_device {
    use super::*;
    use std::fs;

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

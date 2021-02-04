use crate::util;
use reqwest::header;
use std::collections::BTreeMap;

pub struct Client {
    access_token: String,
    base_url: &'static str,
}

impl Client {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            base_url: "https://api.nature.global/",
        }
    }

    pub async fn request(
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
                header::HeaderValue::from_static("/1/application/json"),
            );
            map.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
            map
        };
        let body = util::encode_params(&params);

        let client = reqwest::Client::new();
        client
            .request(method, &format!("{}{}", self.base_url, url))
            .headers(header_map)
            .body(body)
            .send()
            .await
    }
}

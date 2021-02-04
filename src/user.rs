use std::collections::BTreeMap;

use crate::client::Client;
use serde::{Deserialize, Serialize};

/// User information
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    nickname: String,
    superuser: Option<bool>,
}

impl Client {
    pub async fn get_user(&self) -> Result<User, reqwest::Error> {
        let response = self
            .request(reqwest::Method::GET, "/1/users/me", &BTreeMap::new())
            .await?
            .text()
            .await?;
        let user: User = serde_json::from_str(&response).unwrap();
        Ok(user)
    }
}

#[cfg(test)]
mod test_user {
    use super::*;
    use std::fs;

    #[test]
    fn test_user_deserialize() {
        let json = fs::read_to_string("samples/user.json").unwrap();
        let user: User = serde_json::from_str(&json).unwrap();

        assert_eq!(user.id, "test_user_0");
        assert_eq!(user.nickname, "Test user");
    }
}

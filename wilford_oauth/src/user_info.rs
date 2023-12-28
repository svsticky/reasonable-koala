use reqwest::{Client, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub is_admin: bool,
    pub espo_user_id: String,
}

impl UserInfo {
    pub async fn request_info(wilford: &str, token: &str) -> Result<Self> {
        Ok(Client::new()
            .get(format!("{wilford}/api/v1/user/info"))
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

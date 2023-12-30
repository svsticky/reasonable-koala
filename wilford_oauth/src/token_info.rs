use reqwest::{Client, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenInfo {
    pub scope: String,
}

impl TokenInfo {
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scope
            .split(" ")
            .find(|f| (*f).eq(scope))
            .is_some()
    }

    pub async fn request_info(wilford: &str, token: &str) -> Result<TokenInfo> {
        Ok(Client::new()
            .get(format!("{wilford}/api/v1/auth/token-info"))
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

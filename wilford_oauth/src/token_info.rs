use reqwest::{Client, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenInfo {
    pub scopes: Vec<String>,
}

impl TokenInfo {
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().find(|f| f.as_str().eq(scope)).is_some()
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

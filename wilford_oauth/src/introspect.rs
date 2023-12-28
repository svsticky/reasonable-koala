use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct IntrospectionResult {
    pub active: bool,
    pub scope: String,
    pub client_id: String,
    pub username: String,
    pub token_type: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    /// Espo user id
    pub sub: String,
}

impl IntrospectionResult {
    pub async fn new(wilford: &str, cat: &str, token: &str, scope: Option<&str>) -> Result<Self> {
        #[derive(Serialize)]
        struct Query<'a> {
            token: &'a str,
            scope: Option<&'a str>,
        }

        Ok(Client::new()
            .get(&format!("{wilford}/api/oauth/introspect"))
            .query(&Query { token, scope })
            .bearer_auth(cat)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

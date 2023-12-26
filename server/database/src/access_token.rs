use crate::driver::Database;
use crate::generate_string;
use crate::oauth2_client::OAuth2Client;
use sqlx::FromRow;
use sqlx::Result;

#[derive(FromRow)]
pub struct AccessToken {
    pub token: String,
    pub client_id: String,
    pub expires_at: i64,
    pub espo_user_id: String,
}

impl AccessToken {
    fn generate_access_token() -> String {
        generate_string(32)
    }

    fn generate_expires_at() -> i64 {
        (time::OffsetDateTime::now_utc() + time::Duration::hours(1)).unix_timestamp()
    }

    pub async fn new(
        driver: &Database,
        client: &OAuth2Client,
        espo_user_id: String,
    ) -> Result<Self> {
        let token = Self::generate_access_token();
        let expires_at = Self::generate_expires_at();

        sqlx::query("INSERT INTO oauth2_access_tokens (token, client_id, expires_at, espo_user_id) VALUES (?, ?, ?, ?)")
            .bind(&token)
            .bind(&client.client_id)
            .bind(expires_at)
            .bind(&espo_user_id)
            .execute(&**driver)
            .await?;

        Ok(Self {
            token,
            client_id: client.client_id.clone(),
            expires_at,
            espo_user_id,
        })
    }

    pub async fn get_if_valid_for(
        driver: &Database,
        token: &str,
        client: &OAuth2Client,
    ) -> Result<Option<AccessToken>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_access_tokens WHERE token = ? AND client_id = ?")
                .bind(token)
                .bind(&client.client_id)
                .fetch_optional(&**driver)
                .await?
                // Only valid if the token hasn't expired yet
                .map(|token: AccessToken| {
                    let valid = time::OffsetDateTime::now_utc().unix_timestamp() < token.expires_at;
                    valid.then(|| token)
                })
                .unwrap_or(None), // No token found for the client --> not valid
        )
    }
}

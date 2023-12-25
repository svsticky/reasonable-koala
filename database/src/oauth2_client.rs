use sqlx::{FromRow, Result};
use crate::driver::Database;
use crate::generate_string;

#[derive(Debug, Clone, FromRow)]
pub struct OAuth2Client {
    pub name: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub is_internal: bool,
}

#[derive(Debug, Clone, FromRow)]
pub struct OAuth2PendingAuthorization {
    pub id: String,
    pub client_id: String,
    pub scopes: Option<String>,
    pub state: Option<String>,
    pub espo_user_id: Option<String>,
}

#[derive(FromRow)]
pub struct OAuth2AuthorizationCode {
    pub code: String,
    pub client_id: String,
    pub expires_at: i64,
    pub scopes: Option<String>,
}

impl OAuth2Client {
    fn generate_client_id() -> String {
        generate_string(32)
    }

    fn generate_authorization_code() -> String {
        generate_string(32)
    }

    fn generate_client_secret() -> String {
        generate_string(48)
    }

    fn generate_pending_authorization_id() -> String {
        generate_string(16)
    }

    fn generate_authorization_code_expiry() -> i64 {
        (time::OffsetDateTime::now_utc() + time::Duration::minutes(10)).unix_timestamp()
    }

    pub async fn new(driver: &Database, name: String, redirect_uri: String, internal: bool) -> Result<Self> {
        let client_id = Self::generate_client_id();
        let client_secret = Self::generate_client_secret();

        sqlx::query("INSERT INTO oauth2_clients (name, redirect_uri, client_id, client_secret, is_internal) VALUES (?, ?, ?, ?, ?)")
            .bind(&name)
            .bind(&redirect_uri)
            .bind(&client_id)
            .bind(&client_secret)
            .bind(internal)
            .execute(&**driver)
            .await?;

        Ok(Self {
            name,
            redirect_uri,
            client_id,
            client_secret,
            is_internal: internal,
        })
    }

    pub async fn list(driver: &Database) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT * FROM oauth2_clients")
            .fetch_all(&**driver)
            .await?
        )
    }

    pub async fn delete(self, driver: &Database) -> Result<()> {
        sqlx::query("DELETE FROM oauth2_clients WHERE client_id = ?")
            .bind(self.client_id)
            .execute(&**driver)
            .await?;
        Ok(())
    }

    pub async fn get_by_client_id(driver: &Database, client_id: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT * FROM oauth2_clients WHERE client_id = ?")
            .bind(client_id)
            .fetch_optional(&**driver)
            .await?
        )
    }

    pub async fn new_pending_authorization(&self, driver: &Database, scopes: Option<String>, state: Option<String>) -> Result<OAuth2PendingAuthorization> {
        let id = Self::generate_pending_authorization_id();
        sqlx::query("INSERT INTO oauth2_pending_authorizations (id, client_id, scopes, state) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&self.client_id)
            .bind(&scopes)
            .bind(&state)
            .execute(&**driver)
            .await?;

        Ok(OAuth2PendingAuthorization {
            id,
            client_id: self.client_id.clone(),
            scopes,
            state,
            espo_user_id: None,
        })
    }

    pub async fn new_authorization_code(&self, driver: &Database, pending: &OAuth2PendingAuthorization) -> Result<OAuth2AuthorizationCode> {
        let code = Self::generate_authorization_code();
        let expires_at = Self::generate_authorization_code_expiry();

        sqlx::query("INSERT INTO oauth2_authorization_codes (client_id, code, expires_at, scopes) VALUES (?, ?, ?, ?)")
            .bind(&self.client_id)
            .bind(&code)
            .bind(expires_at)
            .bind(&pending.scopes)
            .execute(&**driver)
            .await?;

        Ok(OAuth2AuthorizationCode {
            client_id: self.client_id.clone(),
            code,
            scopes: pending.scopes.clone(),
            expires_at,
        })
    }
}

impl OAuth2PendingAuthorization {
    pub async fn delete(self, driver: &Database) -> Result<()> {
        sqlx::query("DELETE FROM oauth2_pending_authorizations WHERE id = ?")
            .bind(self.id)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    pub async fn get_by_id(driver: &Database, id: &str) -> Result<Option<OAuth2PendingAuthorization>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_pending_authorizations WHERE id = ?")
                .bind(id)
                .fetch_optional(&**driver)
                .await?
        )
    }

    pub async fn set_espo_user_id(&self, driver: &Database, id: &str) -> Result<()> {
        sqlx::query("UPDATE oauth2_pending_authorizations SET espo_user_id = ? WHERE id = ?")
            .bind(id)
            .bind(&self.id)
            .execute(&**driver)
            .await?;
        Ok(())
    }
}

impl OAuth2AuthorizationCode {
    pub async fn get_by_code(driver: &Database, code: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_authorization_codes WHERE code = ?")
                .bind(code)
                .fetch_optional(&**driver)
                .await?
        )
    }
}
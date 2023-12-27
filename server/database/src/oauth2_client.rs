use crate::driver::Database;
use crate::generate_string;
use sqlx::{FromRow, Result};
use std::collections::HashSet;
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow)]
pub struct OAuth2Client {
    pub name: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub is_internal: bool,
}

#[derive(Debug, Clone)]
pub enum OAuth2PendingAuthorization {
    EspoUnauthorized(OAuth2PendingAuthorizationUnauthorized),
    EspoAuthorized(OAuth2PendingAuthorizationAuthorized),
}

impl OAuth2PendingAuthorization {
    pub fn id(&self) -> &String {
        match self {
            Self::EspoAuthorized(v) => &v.id,
            Self::EspoUnauthorized(v) => &v.id,
        }
    }

    pub fn client_id(&self) -> &String {
        match self {
            Self::EspoAuthorized(v) => &v.client_id,
            Self::EspoUnauthorized(v) => &v.client_id,
        }
    }

    pub fn state(&self) -> &Option<String> {
        match self {
            Self::EspoAuthorized(v) => &v.state,
            Self::EspoUnauthorized(v) => &v.state,
        }
    }

    pub fn scopes(&self) -> &Option<String> {
        match self {
            Self::EspoAuthorized(v) => &v.scopes,
            Self::EspoUnauthorized(v) => &v.scopes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OAuth2PendingAuthorizationUnauthorized {
    id: String,
    client_id: String,
    scopes: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OAuth2PendingAuthorizationAuthorized {
    id: String,
    client_id: String,
    scopes: Option<String>,
    state: Option<String>,
    espo_user_id: String,
}

#[derive(FromRow)]
struct _OAuth2PendingAuthorization {
    id: String,
    client_id: String,
    scopes: Option<String>,
    state: Option<String>,
    espo_user_id: Option<String>,
}

#[derive(FromRow)]
pub struct OAuth2AuthorizationCode {
    pub code: String,
    pub client_id: String,
    pub expires_at: i64,
    pub scopes: Option<String>,
    pub espo_user_id: String,
}

#[derive(Clone, Debug, FromRow)]
pub struct AccessToken {
    pub token: String,
    pub client_id: String,
    pub expires_at: i64,
    pub issued_at: i64,
    pub espo_user_id: String,
    pub scopes: Option<String>,
}

#[derive(FromRow)]
pub struct RefreshToken {
    pub token: String,
    pub client_id: String,
    pub espo_user_id: String,
    pub scopes: Option<String>,
}

#[derive(Debug, Error)]
pub enum OAuth2AuthorizationCodeCreationError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Not authorized with EspoCRM yet")]
    Unauthorized,
}

#[derive(Debug, Error)]
pub enum OAuth2PendingAuthorizationSetEspoIdError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Cannot overwrite existing authorization")]
    AlreadyAuthorized,
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

    fn generate_access_token() -> String {
        generate_string(32)
    }

    fn generate_refresh_token() -> String {
        generate_string(32)
    }

    fn generate_access_token_expiry() -> i64 {
        (time::OffsetDateTime::now_utc() + time::Duration::hours(1)).unix_timestamp()
    }

    pub async fn new(
        driver: &Database,
        name: String,
        redirect_uri: String,
        internal: bool,
    ) -> Result<Self> {
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
            .await?)
    }

    pub async fn delete(self, driver: &Database) -> Result<()> {
        sqlx::query("DELETE FROM oauth2_clients WHERE client_id = ?")
            .bind(self.client_id)
            .execute(&**driver)
            .await?;
        Ok(())
    }

    pub async fn get_by_client_id(driver: &Database, client_id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_clients WHERE client_id = ?")
                .bind(client_id)
                .fetch_optional(&**driver)
                .await?,
        )
    }

    pub async fn new_pending_authorization(
        &self,
        driver: &Database,
        scopes: Option<String>,
        state: Option<String>,
    ) -> Result<OAuth2PendingAuthorization> {
        let id = Self::generate_pending_authorization_id();
        sqlx::query("INSERT INTO oauth2_pending_authorizations (id, client_id, scopes, state) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&self.client_id)
            .bind(&scopes)
            .bind(&state)
            .execute(&**driver)
            .await?;

        Ok(OAuth2PendingAuthorization::EspoUnauthorized(
            OAuth2PendingAuthorizationUnauthorized {
                id,
                client_id: self.client_id.clone(),
                scopes,
                state,
            },
        ))
    }

    pub async fn new_authorization_code(
        &self,
        driver: &Database,
        pending: OAuth2PendingAuthorization,
    ) -> std::result::Result<OAuth2AuthorizationCode, OAuth2AuthorizationCodeCreationError> {
        let pending = match pending {
            OAuth2PendingAuthorization::EspoAuthorized(v) => v,
            OAuth2PendingAuthorization::EspoUnauthorized(_) => {
                return Err(OAuth2AuthorizationCodeCreationError::Unauthorized)
            }
        };

        let code = Self::generate_authorization_code();
        let expires_at = Self::generate_authorization_code_expiry();

        let mut tx = driver.begin().await?;

        sqlx::query("INSERT INTO oauth2_authorization_codes (client_id, code, expires_at, scopes, espo_user_id) VALUES (?, ?, ?, ?, ?)")
            .bind(&self.client_id)
            .bind(&code)
            .bind(expires_at)
            .bind(&pending.scopes)
            .bind(&pending.espo_user_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("DELETE FROM oauth2_pending_authorizations WHERE id = ? ")
            .bind(&pending.id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(OAuth2AuthorizationCode {
            client_id: self.client_id.clone(),
            code,
            scopes: pending.scopes.clone(),
            expires_at,
            espo_user_id: pending.espo_user_id,
        })
    }

    pub async fn new_token_pair(
        &self,
        driver: &Database,
        authorization: OAuth2AuthorizationCode,
    ) -> Result<(AccessToken, RefreshToken)> {
        let atoken = Self::generate_access_token();
        let rtoken = Self::generate_refresh_token();
        let expires_at = Self::generate_access_token_expiry();
        let issued_at = OffsetDateTime::now_utc().unix_timestamp();

        let mut tx = driver.begin().await?;

        // Access token
        sqlx::query("INSERT INTO oauth2_access_tokens (token, client_id, expires_at, issued_at, espo_user_id, scopes) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&atoken)
            .bind(&self.client_id)
            .bind(expires_at)
            .bind(issued_at)
            .bind(&authorization.espo_user_id)
            .bind(&authorization.scopes)
            .execute(&mut *tx)
            .await?;

        // Refresh token
        sqlx::query("INSERT INTO oauth2_refresh_tokens (token, client_id, espo_user_id, scopes) VALUES (?, ?, ?, ?)")
            .bind(&rtoken)
            .bind(&self.client_id)
            .bind(&authorization.espo_user_id)
            .bind(&authorization.scopes)
            .execute(&mut *tx)
            .await?;

        // Remove authorization
        sqlx::query("DELETE FROM oauth2_authorization_codes WHERE code = ?")
            .bind(&authorization.code)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok((
            AccessToken {
                token: atoken,
                client_id: self.client_id.clone(),
                expires_at,
                issued_at,
                espo_user_id: authorization.espo_user_id.clone(),
                scopes: authorization.scopes.clone(),
            },
            RefreshToken {
                token: rtoken,
                client_id: self.client_id.clone(),
                espo_user_id: authorization.espo_user_id.clone(),
                scopes: authorization.scopes.clone(),
            },
        ))
    }

    pub async fn new_access_token(
        &self,
        driver: &Database,
        refresh_token: &RefreshToken,
    ) -> Result<AccessToken> {
        let atoken = Self::generate_access_token();
        let expires_at = Self::generate_access_token_expiry();
        let issued_at = OffsetDateTime::now_utc().unix_timestamp();

        sqlx::query("INSERT INTO oauth2_access_tokens (token, client_id, expires_at, issued_at, espo_user_id, scopes) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&atoken)
            .bind(&self.client_id)
            .bind(expires_at)
            .bind(&refresh_token.espo_user_id)
            .bind(&refresh_token.scopes)
            .execute(&**driver)
            .await?;

        Ok(AccessToken {
            token: atoken,
            client_id: self.client_id.clone(),
            scopes: refresh_token.scopes.clone(),
            issued_at,
            expires_at,
            espo_user_id: refresh_token.espo_user_id.clone(),
        })
    }
}

impl AccessToken {
    pub async fn get_by_token(driver: &Database, token: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_access_tokens WHERE token = ?")
                .bind(token)
                .fetch_optional(&**driver)
                .await?,
        )
    }

    pub async fn get_with_validation(
        driver: &Database,
        token: &str,
        client: &OAuth2Client,
    ) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_access_tokens WHERE token = ? AND client_id = ?")
                .bind(token)
                .bind(&client.client_id)
                .fetch_optional(&**driver)
                .await?
                // Only valid if the token hasn't expired yet
                .map(|token: Self| {
                    let valid = time::OffsetDateTime::now_utc().unix_timestamp() < token.expires_at;
                    valid.then(|| token)
                })
                .unwrap_or(None), // No token found for the client --> not valid
        )
    }

    pub fn scopes(&self) -> HashSet<String> {
        self.scopes
            .as_ref()
            .map(|f| f.split(" ").map(|c| c.to_string()).collect::<HashSet<_>>())
            .unwrap_or_default()
    }
}

impl RefreshToken {
    pub async fn get_by_token(driver: &Database, token: &str) -> Result<Option<RefreshToken>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_refresh_tokens WHERE token = ?")
                .bind(token)
                .fetch_optional(&**driver)
                .await?,
        )
    }
}

impl OAuth2PendingAuthorization {
    pub async fn get_by_id(
        driver: &Database,
        id: &str,
    ) -> Result<Option<OAuth2PendingAuthorization>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_pending_authorizations WHERE id = ?")
                .bind(id)
                .fetch_optional(&**driver)
                .await?
                .map(|v: _OAuth2PendingAuthorization| OAuth2PendingAuthorization::from(v)),
        )
    }

    pub async fn set_espo_user_id(
        self,
        driver: &Database,
        espo_user_id: &str,
    ) -> std::result::Result<Self, OAuth2PendingAuthorizationSetEspoIdError> {
        let id = match &self {
            Self::EspoUnauthorized(v) => &v.id,
            Self::EspoAuthorized(_) => {
                return Err(OAuth2PendingAuthorizationSetEspoIdError::AlreadyAuthorized)
            }
        };

        sqlx::query("UPDATE oauth2_pending_authorizations SET espo_user_id = ? WHERE id = ?")
            .bind(espo_user_id)
            .bind(&id)
            .execute(&**driver)
            .await?;

        let new_self = match self {
            Self::EspoUnauthorized(v) => {
                Self::EspoAuthorized(OAuth2PendingAuthorizationAuthorized {
                    id: v.id,
                    client_id: v.client_id,
                    espo_user_id: espo_user_id.to_string(),
                    state: v.state,
                    scopes: v.scopes,
                })
            }
            Self::EspoAuthorized(_) => unreachable!(),
        };

        Ok(new_self)
    }
}

impl OAuth2AuthorizationCode {
    pub async fn get_by_code(driver: &Database, code: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as("SELECT * FROM oauth2_authorization_codes WHERE code = ?")
                .bind(code)
                .fetch_optional(&**driver)
                .await?,
        )
    }
}

impl From<_OAuth2PendingAuthorization> for OAuth2PendingAuthorization {
    fn from(value: _OAuth2PendingAuthorization) -> Self {
        if let Some(espo_user_id) = value.espo_user_id {
            Self::EspoAuthorized(OAuth2PendingAuthorizationAuthorized {
                id: value.id,
                client_id: value.client_id,
                scopes: value.scopes,
                state: value.state,
                espo_user_id,
            })
        } else {
            Self::EspoUnauthorized(OAuth2PendingAuthorizationUnauthorized {
                id: value.id,
                client_id: value.client_id,
                scopes: value.scopes,
                state: value.state,
            })
        }
    }
}

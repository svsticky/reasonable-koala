use actix_web::cookie::time;
use actix_web::web;
use serde::{Deserialize, Serialize};
use tap::TapFallible;
use tracing::warn;
use database::oauth2_client::{OAuth2AuthorizationCode, OAuth2Client};
use crate::routes::appdata::WDatabase;
use crate::routes::oauth::OAuth2ErrorKind;

#[derive(Deserialize)]
pub struct Query {
    grant_type: GrantType,
    code: Option<String>,
    redirect_uri: String,
    client_id: String,
}

#[derive(Deserialize)]
pub enum GrantType {
    #[serde(rename(deserialize = "authorization_code"))]
    AuthorizationCode
}

#[derive(Serialize)]
pub struct Response {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: String,
    scope: String,
}

pub async fn token(database: WDatabase, query: web::Query<Query>) -> Result<web::Json<Response>, OAuth2ErrorKind> {
    let client = OAuth2Client::get_by_client_id(&database, &query.client_id).await
        .tap_err(|e| warn!("{e}"))
        .map_err(|_| OAuth2ErrorKind::ServerError)?
        .ok_or(OAuth2ErrorKind::UnauthorizedClient)?;

    match query.grant_type {
        GrantType::AuthorizationCode => {
            let code = match &query.code {
                Some(c) => c,
                None => return Err(OAuth2ErrorKind::InvalidRequest)
            };

            let authorization = OAuth2AuthorizationCode::get_by_code(&database, code).await
                .tap_err(|e| warn!("{e}"))
                .map_err(|_| OAuth2ErrorKind::ServerError)?
                .ok_or(OAuth2ErrorKind::InvalidGrant)?;

            if time::OffsetDateTime::now_utc().unix_timestamp() > authorization.expires_at {
                return Err(OAuth2ErrorKind::InvalidGrant);
            }

            // TODO generate access & refresh tokens
        }
    }

    todo!()
}
use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::error::{WebError, WebResult};
use actix_web::web;
use database::oauth2_client::OAuth2PendingAuthorization;
use database::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::instrument;

#[derive(Deserialize)]
pub struct Request {
    authorization: String,
    username: String,
    password: String,
    // totp_code: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    status: bool,
    #[serde(skip_serializing_if = "is_false")]
    totp_required: bool,
}

/// Serde helper function
fn is_false(b: &bool) -> bool {
    !b
}

#[instrument(skip_all)]
pub async fn login(
    database: WDatabase,
    config: WConfig,
    payload: web::Json<Request>,
) -> WebResult<web::Json<Response>> {
    // Check if the login is part of a pending authorization
    let authorization = OAuth2PendingAuthorization::get_by_id(&database, &payload.authorization)
        .await?
        .ok_or(WebError::NotFound)?;

    // Fetch the associated user
    let mut user = User::get_by_email(&database, &payload.username)
        .await?
        .ok_or(WebError::Unauthorized)?;

    // Validate password
    if !user
        .check_password(&database, &payload.password, &config.password_pepper)
        .await?
    {
        return Err(WebError::Unauthorized);
    }

    // Mark the authorization as authorized
    // By setting the user id (as it is known now)
    let authorization = authorization
        .set_user_id(&database, &user.id)
        .await
        .map_err(|_| WebError::BadRequest)?;

    // OAuth2 defines `scope` to be all scopes, seperated by a ' ' (space char)
    // Where duplicates can be ignored.
    let scope_set = authorization
        .scopes()
        .clone()
        .map(|s| s.split(" ").map(|c| c.to_string()).collect::<HashSet<_>>())
        .unwrap_or_default();

    // Scopes allowed for the user
    let permitted_scopes = HashSet::from_iter(user.list_permitted_scopes(&database).await?);

    // We always allow OIDC scopes
    let oidc_scopes = oidc_scopes();
    let allowed_scopes = permitted_scopes
        .union(&oidc_scopes)
        .map(|c| c.to_string())
        .collect::<HashSet<_>>();

    // Requested - allowed = The set of requested scopes that aren't allowed
    let disallowed_scopes = scope_set
        .difference(&allowed_scopes)
        .collect::<HashSet<_>>();

    if !disallowed_scopes.is_empty() && !user.is_admin {
        return Err(WebError::Forbidden);
    }

    Ok(web::Json(Response {
        status: true,
        totp_required: false,
    }))
}

fn oidc_scopes() -> HashSet<String> {
    HashSet::from_iter([
        "openid".to_string(),
        "profile".to_string(),
        "email".to_string(),
    ])
}

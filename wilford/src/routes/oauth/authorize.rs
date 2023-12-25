use actix_web::web;
use serde::Deserialize;
use tracing::warn;
use database::oauth2_client::OAuth2Client;
use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::oauth::{OAuth2Error, OAuth2ErrorKind, OAuth2AuthorizationResponse};
use crate::routes::redirect::Redirect;

#[derive(Deserialize)]
pub struct Query {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
}

pub async fn authorize(database: WDatabase, config: WConfig, query: web::Query<Query>) -> OAuth2AuthorizationResponse<Redirect> {
    if query.response_type.ne("code") {
        return OAuth2AuthorizationResponse::Err(OAuth2Error::new(OAuth2ErrorKind::UnsupportedResponseType, &query.redirect_uri, query.state.as_deref()));
    }

    // Get the OAuth2 client
    let client = match OAuth2Client::get_by_client_id(&database, &query.client_id).await {
        Ok(Some(c)) => c,
        Ok(None) => return OAuth2AuthorizationResponse::Err(OAuth2Error::new(OAuth2ErrorKind::UnauthorizedClient, &query.redirect_uri, query.state.as_deref())),
        Err(e) => {
            warn!("{e}");
            return OAuth2AuthorizationResponse::Err(OAuth2Error::new(OAuth2ErrorKind::ServerError, &query.redirect_uri, query.state.as_deref()));
        }
    };

    // Check redirect URI
    if client.redirect_uri.ne(&query.redirect_uri) {
        return OAuth2AuthorizationResponse::Err(OAuth2Error::new(OAuth2ErrorKind::UnauthorizedClient, &query.redirect_uri, query.state.as_deref()));
    }

    // Create authorization
    let pending_authorization = match client.new_pending_authorization(
        &database,
        query.scope.clone(),
        query.state.clone(),
    ).await {
        Ok(pa) => pa,
        Err(e) => {
            warn!("{e}");
            return OAuth2AuthorizationResponse::Err(OAuth2Error::new(OAuth2ErrorKind::ServerError, &query.redirect_uri, query.state.as_deref()));
        }
    };

    // Redirect to login page
    OAuth2AuthorizationResponse::Ok(Redirect::new(
        format!(
            "{}?authorization={}",
            config.http.ui_login_path,
            pending_authorization.id,
        )
    ))
}
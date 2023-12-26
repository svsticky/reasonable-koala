use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::oauth::{OAuth2AuthorizationResponse, OAuth2Error, OAuth2ErrorKind};
use crate::routes::redirect::Redirect;
use actix_web::web;
use database::oauth2_client::OAuth2Client;
use serde::Deserialize;
use tracing::warn;

#[derive(Deserialize)]
pub struct Query {
    response_type: ResponseType,
    client_id: String,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum ResponseType {
    #[serde(rename(deserialize = "code"))]
    /// Authorization Code flow
    /// [RFC6749 Section 4.1](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1)
    Code,
}

pub async fn authorize(
    database: WDatabase,
    config: WConfig,
    query: web::Query<Query>,
) -> OAuth2AuthorizationResponse<Redirect> {
    // Get the OAuth2 client
    let client = match OAuth2Client::get_by_client_id(&database, &query.client_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
                OAuth2ErrorKind::UnauthorizedClient,
                &query.redirect_uri,
                query.state.as_deref(),
            ))
        }
        Err(e) => {
            warn!("{e}");
            return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
                OAuth2ErrorKind::ServerError,
                &query.redirect_uri,
                query.state.as_deref(),
            ));
        }
    };

    // Check redirect URI
    if client.redirect_uri.ne(&query.redirect_uri) {
        return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
            OAuth2ErrorKind::UnauthorizedClient,
            &query.redirect_uri,
            query.state.as_deref(),
        ));
    }

    match query.response_type {
        ResponseType::Code => {
            // Create authorization
            let pending_authorization = match client
                .new_pending_authorization(&database, query.scope.clone(), query.state.clone())
                .await
            {
                Ok(pa) => pa,
                Err(e) => {
                    warn!("{e}");
                    return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
                        OAuth2ErrorKind::ServerError,
                        &query.redirect_uri,
                        query.state.as_deref(),
                    ));
                }
            };

            // Redirect to login page
            OAuth2AuthorizationResponse::Ok(Redirect::new(format!(
                "{}?authorization={}",
                config.http.ui_login_path,
                pending_authorization.id(),
            )))
        }
    }
}

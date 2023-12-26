use crate::routes::appdata::WDatabase;
use crate::routes::error::{WebError, WebResult};
use crate::routes::oauth::{OAuth2AuthorizationResponse, OAuth2Error, OAuth2ErrorKind};
use crate::routes::redirect::Redirect;
use actix_web::web;
use database::oauth2_client::{
    OAuth2AuthorizationCodeCreationError, OAuth2Client, OAuth2PendingAuthorization,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Query {
    authorization: String,
    grant: bool,
}

pub async fn authorize(
    database: WDatabase,
    query: web::Query<Query>,
) -> WebResult<OAuth2AuthorizationResponse<Redirect>> {
    let pending_authorization =
        OAuth2PendingAuthorization::get_by_id(&database, &query.authorization)
            .await?
            .ok_or(WebError::NotFound)?;

    let client = OAuth2Client::get_by_client_id(&database, &pending_authorization.client_id())
        .await?
        .ok_or(WebError::NotFound)?;

    if !query.grant {
        return Ok(OAuth2AuthorizationResponse::Err(OAuth2Error::new(
            OAuth2ErrorKind::AccessDenied,
            &client.redirect_uri,
            pending_authorization.state().as_deref(),
        )));
    }

    let state = pending_authorization.state().clone();
    let authorization = client
        .new_authorization_code(&database, pending_authorization)
        .await
        .map_err(|e| match e {
            OAuth2AuthorizationCodeCreationError::Sqlx(e) => WebError::Database(e),
            OAuth2AuthorizationCodeCreationError::Unauthorized => WebError::InvalidInternalState,
        })?;

    #[derive(Serialize)]
    struct RedirectQuery {
        code: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,
    }

    let redirect_uri = format!(
        "{}?{}",
        client.redirect_uri,
        serde_qs::to_string(&RedirectQuery {
            code: authorization.code,
            state,
        })
        .expect("Serializing query string"),
    );

    Ok(OAuth2AuthorizationResponse::Ok(Redirect::new(redirect_uri)))
}

use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebError, WebResult};
use crate::routes::v1::MANAGE_SCOPE;
use actix_web::web;
use database::constant_access_tokens::ConstantAccessToken;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    tokens: Vec<Cat>,
}

#[derive(Serialize)]
pub struct Cat {
    name: String,
    token: String,
}

pub async fn list(database: WDatabase, auth: Auth) -> WebResult<web::Json<Response>> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebError::Forbidden);
    }

    let tokens = ConstantAccessToken::list(&database)
        .await?
        .into_iter()
        .map(|c| Cat {
            name: c.name,
            token: c.token,
        })
        .collect::<Vec<_>>();

    Ok(web::Json(Response { tokens }))
}

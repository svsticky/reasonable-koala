use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::empty::Empty;
use crate::routes::error::{WebError, WebResult};
use crate::routes::v1::MANAGE_SCOPE;
use actix_web::web;
use database::constant_access_tokens::ConstantAccessToken;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    token: String,
}

pub async fn remove(
    database: WDatabase,
    auth: Auth,
    payload: web::Json<Request>,
) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebError::Forbidden);
    }

    let cat = ConstantAccessToken::get_by_token(&database, &payload.token)
        .await?
        .ok_or(WebError::NotFound)?;
    cat.revoke(&database).await?;

    Ok(Empty)
}

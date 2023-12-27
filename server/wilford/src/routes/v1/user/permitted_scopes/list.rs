use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebError, WebResult};
use crate::routes::v1::MANAGE_SCOPE;
use actix_web::web;
use database::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response {
    scopes: Vec<String>,
}

#[derive(Deserialize)]
pub struct Query {
    /// Espo user ID
    user: String,
}

pub async fn list(
    database: WDatabase,
    auth: Auth,
    query: web::Query<Query>,
) -> WebResult<web::Json<Response>> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebError::Forbidden);
    }

    let user = User::get_by_id(&database, &query.user)
        .await?
        .ok_or(WebError::NotFound)?;
    let scopes = user.list_permitted_scopes(&database).await?;

    Ok(web::Json(Response { scopes }))
}

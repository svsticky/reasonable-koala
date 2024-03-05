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
    /// User ID
    user: Option<String>,
}

pub async fn list(
    database: WDatabase,
    auth: Auth,
    query: web::Query<Query>,
) -> WebResult<web::Json<Response>> {
    // Admin privileges are only required if the query is not for the current user
    let user_id = match &query.user {
        Some(user_id) => {
            if !auth.has_scope(MANAGE_SCOPE) && user_id.ne(&auth.user_id) {
                return Err(WebError::Forbidden);
            } else {
                user_id
            }
        }
        None => &auth.user_id,
    };

    let user = User::get_by_id(&database, user_id)
        .await?
        .ok_or(WebError::NotFound)?;
    let scopes = user.list_permitted_scopes(&database).await?;

    Ok(web::Json(Response { scopes }))
}

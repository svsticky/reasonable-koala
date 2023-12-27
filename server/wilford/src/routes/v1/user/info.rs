use crate::routes::auth::Auth;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    name: String,
    is_admin: bool,
    espo_user_id: String,
}

pub async fn info(auth: Auth) -> web::Json<Response> {
    web::Json(Response {
        name: auth.name,
        espo_user_id: auth.espo_user_id,
        is_admin: auth.is_admin,
    })
}

use crate::espo::user::{EspoUser, LoginStatus};
use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::error::{WebError, WebResult};
use actix_web::web;
use database::oauth2_client::OAuth2PendingAuthorization;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    authorization: String,
    username: String,
    password: String,
    totp_code: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    status: bool,
    totp_required: bool,
}

pub async fn login(
    database: WDatabase,
    config: WConfig,
    payload: web::Json<Request>,
) -> WebResult<web::Json<Response>> {
    let authorization = OAuth2PendingAuthorization::get_by_id(&database, &payload.authorization)
        .await?
        .ok_or(WebError::NotFound)?;

    let login = EspoUser::try_login(
        &config.espo.host,
        &payload.username,
        &payload.password,
        payload.totp_code.as_deref(),
    )
    .await
    .map_err(|e| WebError::Espo(e))?;

    match login {
        LoginStatus::Ok(id) => {
            authorization
                .set_espo_user_id(&database, &id)
                .await
                .map_err(|_| WebError::BadRequest)?;
            Ok(web::Json(Response {
                status: true,
                totp_required: false,
            }))
        }
        LoginStatus::SecondStepRequired => Ok(web::Json(Response {
            status: false,
            totp_required: true,
        })),
        LoginStatus::Err => Ok(web::Json(Response {
            status: false,
            totp_required: false,
        })),
    }
}

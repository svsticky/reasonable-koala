use crate::actix::config::WilfordConfig;
use crate::actix::error::OAuth2Error;
use crate::token_info::TokenInfo;
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::{web, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;
use tracing::warn;
use crate::user_info::UserInfo;

/// Extractor for Actix-Web.
/// This middleware extracts the Bearer token (provided in the `Authorization` header)
/// and checks it with the Wilford identity provider.
///
///
/// # Panics
/// If no [actix_web::web::Data<WilfordConfig>] is stored in [actix_web::App::app_data].
pub struct WilfordAuth {
    pub token_info: TokenInfo,
    pub token: String,
    wilford_host: String,
}

impl WilfordAuth {
    pub fn has_scope<S: AsRef<str>>(&self, scope: S) -> bool {
        self.token_info.has_scope(scope.as_ref())
    }

    pub async fn user_info(&self) -> reqwest::Result<UserInfo> {
        UserInfo::request_info(&self.wilford_host, &self.token).await
    }

}


impl FromRequest for WilfordAuth {
    type Error = OAuth2Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = extract_token(&req).ok_or(OAuth2Error::InvalidRequest)?;

            let wilford: &web::Data<WilfordConfig> = req
                .app_data()
                .expect("Getting WilfordConfig from AppData. Is it configured?");
            let token_info = TokenInfo::request_info(&wilford.wilford, &token)
                .await
                .map_err(|e| match e.status() {
                    Some(StatusCode::UNAUTHORIZED) => OAuth2Error::InvalidToken,
                    _ => {
                        warn!("{e}");
                        OAuth2Error::Reqwest(e)
                    },
                })?;

            Ok(Self {
                token_info,
                token,
                wilford_host: wilford.wilford.clone(),
            })
        })
    }
}

fn extract_token(req: &HttpRequest) -> Option<String> {
    let value = req
        .headers()
        .get("Authorization")
        .map(|header_vaue| header_vaue.to_str())?
        .ok()?;

    if value.starts_with("Bearer ") {
        return Some(value.replace("Bearer ", ""));
    }

    None
}

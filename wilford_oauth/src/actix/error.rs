use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuth2Error {
    #[error("invalid_request")]
    InvalidRequest,
    #[error("invalid_token")]
    InvalidToken,
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}

impl ResponseError for OAuth2Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::InvalidRequest => StatusCode::UNAUTHORIZED,
            Self::Reqwest(_) => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let error = match self {
            Self::InvalidToken => "invalid_token",
            Self::InvalidRequest => "invalid_request",
            Self::Reqwest(_) => "server_error",
        };

        #[derive(Serialize)]
        struct Response<'a> {
            error: &'a str,
        }

        HttpResponse::build(self.status_code()).json(&Response { error })
    }
}

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

pub type WebResult<T> = Result<T, WebError>;

#[derive(Debug, Error)]
pub enum WebError {
    #[error("Not found")]
    NotFound,
    #[error("Bad request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid internal state")]
    InvalidInternalState,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    Database(#[from] database::driver::Error),
    #[error("{0}")]
    DatabaseUser(#[from] database::user::UserError),
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::InvalidInternalState => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseUser(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

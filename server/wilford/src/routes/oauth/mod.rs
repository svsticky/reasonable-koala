use actix_route_config::Routable;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use std::fmt::Display;

mod authorize;
mod token;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/oauth")
                .route("/authorize", web::get().to(authorize::authorize))
                .route("/token", web::post().to(token::token)),
        );
    }
}

pub enum OAuth2AuthorizationResponse<T: Responder> {
    Ok(T),
    Err(OAuth2Error),
}

impl<T: Responder<Body = BoxBody>> Responder for OAuth2AuthorizationResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Self::Ok(v) => v.respond_to(req),
            Self::Err(e) => {
                #[derive(Serialize)]
                struct Query {
                    error: String,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    state: Option<String>,
                }

                let qs = serde_qs::to_string(&Query {
                    state: e.state,
                    error: e.kind.to_string(),
                })
                .expect("Serializing query");

                let url = format!("{}?{qs}", e.redirect_uri);

                HttpResponse::SeeOther()
                    .insert_header(("Location", url))
                    .finish()
            }
        }
    }
}

pub struct OAuth2Error {
    kind: OAuth2ErrorKind,
    redirect_uri: String,
    state: Option<String>,
}

impl OAuth2Error {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(
        kind: OAuth2ErrorKind,
        redirect_uri: S1,
        state: Option<S2>,
    ) -> Self {
        OAuth2Error {
            kind,
            redirect_uri: redirect_uri.as_ref().to_string(),
            state: state.map(|s| s.as_ref().to_string()),
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum OAuth2ErrorKind {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    InvalidGrant,
    UnsupportedGrantType,
}

impl Display for OAuth2ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidRequest => "invalid_request",
                Self::UnauthorizedClient => "unauthorized_client",
                Self::AccessDenied => "access_denied",
                Self::UnsupportedResponseType => "unsupported_response_type",
                Self::InvalidScope => "invalid_scope",
                Self::ServerError => "server_error",
                Self::InvalidGrant => "invalid_grant",
                Self::UnsupportedGrantType => "unsupported_grant_type",
            }
        )
    }
}

impl ResponseError for OAuth2ErrorKind {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        #[derive(Serialize)]
        struct Response {
            error: String,
        }

        HttpResponse::build(self.status_code()).json(&Response {
            error: self.to_string(),
        })
    }
}

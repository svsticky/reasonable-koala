use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod oauth;
mod v1;
mod appdata;
mod error;
mod empty;
mod auth;
mod redirect;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config
            .service(web::scope("/static")
                // TODO
            )
            .service(web::scope("/api")
                .configure(v1::Router::configure)
                .configure(oauth::Router::configure)
        );
    }
}
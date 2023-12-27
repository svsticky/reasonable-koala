use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod auth;
mod cat;
mod clients;
mod user;

pub const MANAGE_SCOPE: &str = "wilford.manage";

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/v1")
                .configure(clients::Router::configure)
                .configure(auth::Router::configure)
                .configure(user::Router::configure)
                .configure(cat::Router::configure),
        );
    }
}

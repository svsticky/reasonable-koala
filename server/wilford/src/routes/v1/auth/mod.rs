use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod authorization_info;
mod authorize;
mod login;
mod token_info;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/auth")
                .route("/login", web::post().to(login::login))
                .route("/authorize", web::get().to(authorize::authorize))
                .route(
                    "/authorization-info",
                    web::get().to(authorization_info::authorization_info),
                )
                .route("/token-info", web::get().to(token_info::token_info)),
        );
    }
}

use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod info;
mod list;
mod permitted_scopes;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/user")
                .configure(permitted_scopes::Router::configure)
                .route("/info", web::get().to(info::info))
                .route("/list", web::get().to(list::list)),
        );
    }
}

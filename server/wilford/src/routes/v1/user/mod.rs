use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod info;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/user").route("/info", web::get().to(info::info)));
    }
}

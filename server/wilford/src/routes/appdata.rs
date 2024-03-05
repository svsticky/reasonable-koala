use crate::config::Config;
use actix_web::web;
use database::driver::Database;

pub type WDatabase = web::Data<Database>;
pub type WConfig = web::Data<Config>;

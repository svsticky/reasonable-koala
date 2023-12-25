use actix_web::web;
use espocrm_rs::EspoApiClient;
use database::driver::Database;
use crate::config::Config;

pub type WDatabase = web::Data<Database>;
pub type WConfig = web::Data<Config>;
pub type WEspo = web::Data<EspoApiClient>;
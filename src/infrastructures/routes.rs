use actix_web::web;
use crate::adapters::handlers::vocabulary;

pub fn setup_routing(config: &mut web::ServiceConfig) {
    config.service(vocabulary::hello);
}    



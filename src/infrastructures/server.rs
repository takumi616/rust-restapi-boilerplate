use actix_web::{App, HttpServer};
use actix_web::web::ServiceConfig;
use crate::infrastructures::routes;

pub async fn run(port: &str) -> std::io::Result<()> {
    let listen_address: String = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new().configure(move |cfg: &mut ServiceConfig| routes::setup_routing(cfg))
    })
    .bind(&listen_address)?
    .run()
    .await
}



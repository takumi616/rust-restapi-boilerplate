use crate::infrastructures::server::Server;
use std::env;
use std::env::VarError;

mod infrastructures;

#[actix_web::main]
async fn main() {
    let port: String = env::var("APP_CONTAINER_PORT").unwrap_or_else(|e: VarError| {
        eprintln!("Couldn't read app container port: {}", e);
        std::process::exit(1);
    });
    println!("Server will run on port: {}", port);

    let server: Server = Server::new(port);
    if let Err(e) = server.run().await {
        eprintln!("Couldn't run http server: {}", e);
        std::process::exit(1);
    }
}

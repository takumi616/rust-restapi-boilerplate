use std::env;
use std::env::VarError;

use crate::infrastructures::server;

mod infrastructures;
mod adapters;

#[actix_web::main]
async fn main() {
    //env::var returns Result<String, VarError> 
    //Ok(String)、Err(VarError) 
    let port: String = env::var("APP_CONTAINER_PORT").unwrap_or_else(|e: VarError| {
        eprintln!("Couldn't read app container port: {}", e);
        std::process::exit(1);
    });
    println!("Server will run on port: {}", port);

    //Run server
    if let Err(e) = server::run(port.as_str()).await {
        eprintln!("Couldn't run http server: {}", e);
        std::process::exit(1);
    }
}

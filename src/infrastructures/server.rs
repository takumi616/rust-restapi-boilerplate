use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub struct Server {
    pub port: String,
}

impl Server {
    pub fn new(port: String) -> Self {
        Server {
            port,
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listen_address = format!("0.0.0.0:{}", self.port);

        HttpServer::new(|| {
            App::new()
                .service(hello)
        })
        .bind(&listen_address)?
        .run()
        .await
    }
}


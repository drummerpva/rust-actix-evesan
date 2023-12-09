use actix_web::{web, App, HttpServer};
use rust_actix::{config::read_config, infra::endpoints};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = read_config();
    HttpServer::new(|| App::new().service(web::scope("/api").configure(endpoints::config)))
        .bind((config.host, config.port))?
        .run()
        .await
}

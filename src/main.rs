use actix_web::{web, App, HttpServer};
use rust_actix::infra::endpoints::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").configure(config)))
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}

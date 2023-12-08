use actix_web::{get, web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(|| async { return "Hello World!" })))
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}

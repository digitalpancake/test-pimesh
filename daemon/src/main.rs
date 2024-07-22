use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Web API on http://localhost:8080");
    HttpServer::new(|| App::new().service(user::log))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

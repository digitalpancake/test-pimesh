use actix_web::{App, HttpServer};

mod db;
mod log;
mod endpoints;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Web API on http://localhost:8080");

    match db::create_db() {
        Ok(_) => {
            println!("Successfully created db")
        },
        Err(e) => {
            println!("Error accessing db: {e}")
        }
    };

    HttpServer::new(|| App::new().service(endpoints::log))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

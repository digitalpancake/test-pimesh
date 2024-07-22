#[derive(Debug, serde::Deserialize)]
struct UserLogParams {
    user: String,
    key: String,
}

#[actix_web::post("/user/log")]
pub async fn log(params: actix_web::web::Path<UserLogParams>) -> impl actix_web::Responder {
    println!("Received on endpoint /user/log");
    println!("User: {}, Key: {}", params.user, params.key);
    actix_web::HttpResponse::Ok().body("Log Received")
}

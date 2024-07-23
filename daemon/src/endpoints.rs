use crate::log::Loggable;

#[derive(Debug, serde::Deserialize)]
struct UserLogParams {
    user: String,
    key: String,
}

#[actix_web::post("/api/user/log")]
pub async fn log(params: actix_web::web::Query<UserLogParams>, json: actix_web::web::Json<crate::log::UserLog>) -> impl actix_web::Responder {
    println!("User: {}, Key: {}", params.user, params.key);
    println!("Data: {}", json.0.data.clone().unwrap_or("No Data".into()));
    if let Err(_) = json.0.log() {
        eprintln!("Error with sql execution");
        return actix_web::HttpResponse::Locked();
    } else {
        println!("Success");
    }
    actix_web::HttpResponse::Ok()
}

#[actix_web::post("/api/user/login")]
pub async fn login() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}

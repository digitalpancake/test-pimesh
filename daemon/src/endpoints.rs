#![allow(dead_code)]
use crate::log::Loggable;

use crate::user::get_user;

#[derive(Debug, serde::Deserialize)]
struct UserLogParams {
    user: String,
    key: String,
}

#[actix_web::post("/api/user/log")]
pub async fn log(
    params: actix_web::web::Query<UserLogParams>,
    json: actix_web::web::Json<crate::log::UserLog>,
) -> impl actix_web::Responder {
    match get_user(params.0.user) {
        Ok(num) => if num == 1 {},
        Err(e) => {
            println!("Error: {e}");
            return actix_web::HttpResponse::BadRequest();
        }
    };
    if let Err(e) = json.0.log() {
        eprintln!("Error with sql execution: {e}");
        return actix_web::HttpResponse::Locked();
    }
    actix_web::HttpResponse::Ok()
}

#[actix_web::post("/api/user/login")]
pub async fn login() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}

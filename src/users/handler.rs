use crate::users::model::UserData;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
async fn find_all_users() -> impl Responder {
    HttpResponse::Ok().body("hi all users")
}

#[post("/")]
async fn create_user(user_data: web::Json<UserData>) -> impl Responder {
    HttpResponse::Ok().body(user_data)
}

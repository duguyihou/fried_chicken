use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn find_all_users() -> impl Responder {
    HttpResponse::Ok().body("hi all users")
}


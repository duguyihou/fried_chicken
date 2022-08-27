use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("hi user")
}
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(hi));
}

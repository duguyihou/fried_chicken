use actix_web::web;

mod handler;
mod model;

pub fn module(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(handler::find_all_users));
}

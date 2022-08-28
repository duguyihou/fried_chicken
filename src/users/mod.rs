use actix_web::web;

mod controller;

pub fn module(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(controller::find_all_users));
}

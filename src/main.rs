use actix_web::{App, HttpServer};

mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(users::route))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

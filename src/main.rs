use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use std::io::Result;

mod handler;
pub mod qr_generator;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(handler::home)
            .service(handler::qr_list)
            .service(handler::new)
            .service(handler::create)
            .service(handler::show)
            .service(actix_files::Files::new("/static", "./static"))
            .default_service(web::to(handler::not_found))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

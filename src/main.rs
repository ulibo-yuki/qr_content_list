use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use std::{fs::OpenOptions, io::Result};

mod handler;
pub mod qr_generator;

#[actix_rt::main]
async fn main() -> Result<()> {
    make_data_json();
    let ip = if cfg!(debug_assertions) {
        "127.0.0.1:8000"
    } else {
        "192.168.1.100:8080"
    };
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(handler::favicon)
            .service(handler::home)
            .service(handler::qr_list)
            .service(handler::new)
            .service(handler::create)
            .service(handler::destroy)
            .service(handler::show)
            .service(actix_files::Files::new("/static", "./static"))
            .default_service(web::to(handler::not_found))
            .wrap(Logger::default())
    })
    .bind(ip)?
    .run()
    .await
}

fn make_data_json() {
    let _ = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open("data.json")
        .unwrap();
}

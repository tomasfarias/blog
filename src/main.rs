#[macro_use]
extern crate diesel;
extern crate lazy_static;

use std::{env, io, process};

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv;
use tera::Tera;

mod errors;
mod routes;
mod db;
mod models;
mod config;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined");
    let pool = db::init_pool(&database_url).expect("Failed to create database connection pool");

    let app = move || {
        let templates = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let error_handlers = errors::init_error_handlers();

        App::new()
            .data(templates)
            .data(pool.clone())
            .wrap(error_handlers)
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(routes::blog)))
            .service(web::resource("/posts/{slug}").route(web::get().to(routes::post)))
            .service(fs::Files::new("/static", "static/"))
    };
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}

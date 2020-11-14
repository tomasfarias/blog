#[macro_use]
extern crate diesel;

use std::{env, io, process};

use actix_files as fs;
use actix_web::middleware::{Logger};
use actix_web::{get, http, web, App, HttpServer, Responder};
use tera::Tera;

mod errors;
mod routes;
mod db;
mod models;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined");
    let pool = db::init_pool(&database_url).expect("Failed to create database connection pool");

    let app = move || {
        let mut templates = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };

        App::new()
            .data(templates)
            .data(pool.clone())
            .wrap(routes::error_handlers())
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(routes::index)))
            .service(web::resource("/index").route(web::get().to(routes::index)))
            .service(web::resource("/blog").route(web::get().to(routes::blog)))
            .service(web::resource("/blog/post/{slug}").route(web::get().to(routes::blog)))
            .service(web::resource("/hireme").route(web::get().to(routes::hire_me)))
            .service(fs::Files::new("/static", "static/"))
    };
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}

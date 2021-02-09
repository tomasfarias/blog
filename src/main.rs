#[macro_use]
extern crate diesel;
extern crate lazy_static;

use std::{env, io, process};

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use dotenv;
use tera::Tera;
use time::Duration;

mod errors;
mod routes;
mod db;
mod models;
mod auth;
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

        let domain: String = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
        let identity_service = IdentityService::new(
            CookieIdentityPolicy::new(config::CONFIG.secret_key.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain.as_str())
                .max_age_time(Duration::days(1))
                .secure(true),
        );

        App::new()
            .data(templates)
            .data(pool.clone())
            .wrap(error_handlers)
            .wrap(Logger::default())
            .wrap(identity_service)
            .service(web::resource("/").route(web::get().to(routes::index)))
            .service(web::resource("/index").route(web::get().to(routes::index)))
            .service(web::resource("/blog").route(web::get().to(routes::blog)))
            .service(web::resource("/blog/{slug}").route(web::get().to(routes::post)))
            .service(web::resource("/write")
                     .route(web::get().to(routes::write))
                     .route(web::post().to(routes::create)))
            .service(web::resource("/hireme").route(web::get().to(routes::hire_me)))
            .service(fs::Files::new("/static", "static/"))
    };
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}

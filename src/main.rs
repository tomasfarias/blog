use std::{env, io, process};

use actix_files as fs;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{get, http, web, App, HttpServer, Responder};
use tera::Tera;

mod errors;
mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    env_logger::init();
    let app = move || {
        let mut templates = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };

        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                errors::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, errors::bad_request)
            .handler(http::StatusCode::NOT_FOUND, errors::not_found);

        App::new()
            .data(templates)
            .wrap(Logger::default())
            .wrap(error_handlers)
            .service(web::resource("/").route(web::get().to(routes::index)))
            .service(web::resource("/index").route(web::get().to(routes::index)))
            .service(web::resource("/blog").route(web::get().to(routes::blog)))
            .service(web::resource("/blog/post/{id}").route(web::get().to(routes::blog)))
            .service(fs::Files::new("/static", "static/"))
    };
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}

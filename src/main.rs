use std::{env, io, process};

use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::{Logger};
use tera::Tera;

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

        App::new()
            .data(templates)
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(routes::index)))
            .service(web::resource("/index").route(web::get().to(routes::index)))
            .service(fs::Files::new("/static", "static/"))
            
    };
    HttpServer::new(app)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

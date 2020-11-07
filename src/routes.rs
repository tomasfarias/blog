use actix_web::{error, web, Error, HttpResponse, Result};
use tera::{Context, Tera};

pub async fn index(
    tmpl: web::Data<Tera>
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

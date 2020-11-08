use actix_web::{error, web, Error, HttpResponse, Result};
use tera::{Context, Tera};

pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn blog(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let posts: Vec<String> = Vec::new();
    context.insert("posts", &posts);
    
    let rendered = tmpl
        .render("blog.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn post(tmpl: web::Data<Tera>, post_id: web::Path<u32>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();

    let rendered = tmpl
        .render("post.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn hire_me(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();

    let rendered = tmpl
        .render("hireme.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

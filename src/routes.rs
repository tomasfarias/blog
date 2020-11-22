use log::error;

use actix_web::{error, web, Error, HttpResponse, Result};
use actix_web::http::header;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::db;

pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let context = Context::new();

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn blog(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let posts = web::block(move || db::select_last_n_posts(10, &pool))
        .await
        .map_err(|e| {
            error!("Failed to select posts: {}", e);
            e
        })?;
    context.insert("posts", &posts);

    let rendered = tmpl
        .render("blog.html.tera", &context)
        .map_err(|e| {
            error!("Failed to render template: {}", e);
            error::ErrorInternalServerError("An unexpected error has ocurred")
        })?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn post(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    post_slug: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let post = web::block(move || db::select_post_with_slug(&post_slug, &pool))
        .await
        .map_err(|e| {
            error!("Failed to select post: {}", e);
            e
        })?;
    context.insert("post", &post);

    let rendered = tmpl
        .render("post.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Deserialize)]
pub struct WriteForm {
    title: String,
    body: String,
    published: String,
}

impl WriteForm {
    pub fn is_published(&self) -> bool{
        match self.published.as_str() {
            "on" => true,
            _ => false,
        }
    }
}

pub async fn create(
    form: web::Form<WriteForm>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let post = web::block(
        move || db::insert_new_post(&form.title, &form.body, form.is_published(), &pool)
    ).await?;

    let redirect = HttpResponse::Found()
        .header(header::LOCATION, format!("/blog/{}", post.slug))
        .finish();
    Ok(redirect)
}

pub async fn write(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let context = Context::new();

    let rendered = tmpl
        .render("write.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn hire_me(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let context = Context::new();

    let rendered = tmpl
        .render("hireme.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

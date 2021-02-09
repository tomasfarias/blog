use log;

use crate::auth::{verify};
use actix_identity::Identity;
use actix_web::{error, web, Error, HttpResponse, Result, http};
use actix_web::http::header;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::db;

pub async fn index(tmpl: web::Data<Tera>, id: Identity) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    if let Some(identity) = id.identity() {
        context.insert("identity", &identity);
    }

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn blog(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    id: Identity,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let posts = web::block(move || db::select_last_n_posts(10, &pool))
        .await
        .map_err(|e| {
            log::error!("Failed to select posts: {}", e);
            e
        })?;
    context.insert("posts", &posts);
    if let Some(identity) = id.identity() {
        context.insert("identity", &identity);
    }

    let rendered = tmpl
        .render("blog.html.tera", &context)
        .map_err(|e| {
            log::error!("Failed to render template: {}", e);
            error::ErrorInternalServerError("An unexpected error has ocurred")
        })?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn post(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    post_slug: web::Path<String>,
    id: Identity,
) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    let post = web::block(move || db::select_post_with_slug(&post_slug, &pool))
        .await
        .map_err(|e| {
            log::error!("Failed to select post: {}", e);
            e
        })?;
    context.insert("post", &post);
    if let Some(identity) = id.identity() {
        context.insert("identity", &identity);
    }

    let rendered = tmpl
        .render("post.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login(
    id: Identity,
    login: web::Form<LoginForm>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse, Error> {
    let res = web::block(move || db::select_user_with_email(&login.email, &pool))
        .await;

    match res {
        Ok(user) => {
            if let Ok(matching) = verify(&user.hash, &login.password) {
                if matching {
                   id.remember(user.id.to_string());
                   return Ok(
                       HttpResponse::Found()
                           .header(http::header::LOCATION, "/")
                           .finish()
                   )
               }
            }
        },
        Err(err) => {
            return Err(error::ErrorUnauthorized("Invalid username or password"))
        },
    }
    Err(error::ErrorUnauthorized("Invalid username or password"))
}

pub async fn logout(
    id: Identity,
) -> Result<HttpResponse, Error> {
    id.forget();
    Ok(HttpResponse::Found()
       .header(http::header::LOCATION, "/")
       .finish())
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
    id: Identity,
) -> Result<HttpResponse, Error> {
    if id.identity().is_none() {
        return error::ErrorUnauthorized()
    }

    let post = web::block(
        move || db::insert_new_post(&form.title, &form.body, form.is_published(), &pool)
    ).await?;

    let redirect = HttpResponse::Found()
        .header(header::LOCATION, format!("/blog/{}", post.slug))
        .finish();
    Ok(redirect)
}

pub async fn write(tmpl: web::Data<Tera>, id: Identity) -> Result<HttpResponse, Error> {
    if id.identity().is_none() {
        return error::ErrorUnauthorized()
    }

    let mut context = Context::new();
    context.insert("identity", &id.identity().unwrap());

    let rendered = tmpl
        .render("write.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn hire_me(tmpl: web::Data<Tera>, id: Identity) -> Result<HttpResponse, Error> {
    let mut context = Context::new();

    let rendered = tmpl
        .render("hireme.html.tera", &context)
        .map_err(|_| error::ErrorInternalServerError("An unexpected error has ocurred"))?;

    Ok(HttpResponse::Ok().body(rendered))
}

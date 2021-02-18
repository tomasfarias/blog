use log;

use actix_web::{error, error::BlockingError, web, HttpResponse, Result, dev::HttpResponseBuilder, http::header, http::StatusCode};
use derive_more::{Display, Error};
use tera::{Context, Tera};
use serde::Deserialize;

use crate::db::{self, DatabaseError};


#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "An internal error ocurred. Please try again later.")]
    InternalError,
    #[display(fmt = "The post you are looking for does not exist.")]
    PostNotFound,
}


impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::PostNotFound => StatusCode::NOT_FOUND,
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Deserialize)]
pub struct PageQuery {
    page: Option<i64>,
}

pub async fn blog(
    pool: web::Data<db::PgPool>,
    query: web::Query<PageQuery>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, ServerError> {
    let offset = query.page.unwrap_or(0) * 15;
    let posts = web::block(move || db::select_last_n_posts(10, offset, &pool))
        .await
        .map_err(|e| {
            match e {
                BlockingError::Error(DatabaseError::ConnectionPoolError(_)) => {
                    log::error!("Error with connection pool: {}", e);
                    ServerError::InternalError
                },
                _ => {
                    log::error!("Database error: {}", e);
                    ServerError::InternalError
                }
            }
        })?;

    let mut context = Context::new();
    context.insert("posts", &posts);

    let rendered = tmpl
        .render("blog.html.tera", &context)
        .map_err(|e| {
            log::error!("Failed to render template: {}", e);
            ServerError::InternalError
        })?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn post(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    post_slug: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let post = web::block(move || db::select_post_with_slug(&post_slug, &pool))
        .await
        .map_err(|e| {
            match e {
                BlockingError::Error(DatabaseError::ConnectionPoolError(_)) => {
                    log::error!("Error with connection pool: {}", e);
                    ServerError::InternalError
                },
                BlockingError::Error(DatabaseError::NotFound(_)) => {
                    log::error!("Post not found: {}", e);
                    ServerError::PostNotFound
                },
                _ => {
                    log::error!("Database error: {}", e);
                    ServerError::InternalError
                }
            }
        })?;

    let mut context = Context::new();
    context.insert("post", &post);

    let rendered = tmpl
        .render("post.html.tera", &context)
        .map_err(|e| {
            log::error!("Failed to render template: {}", e);
            ServerError::InternalError
        })?;

    Ok(HttpResponse::Ok().body(rendered))
}

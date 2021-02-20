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
    let page = query.page.unwrap_or(1);

    let conn = db::get_conn(&pool)
        .map_err(|e| {
            log::error!("Database error: {}", e);
            ServerError::InternalError
        })?;
    let posts = web::block(move || db::select_last_n_posts(20, (page - 1) * 15, conn))
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

    let conn = db::get_conn(&pool)
        .map_err(|e| {
            log::error!("Database error: {}", e);
            ServerError::InternalError
        })?;
    let total_posts = web::block(move || db::count_total_posts(conn))
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
    let total_pages = total_posts / 15;

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("page", &page);
    context.insert("total_pages", &total_pages);

    let rendered = tmpl
        .render("blog.html.tera", &context)
        .map_err(|e| {
            log::error!("Failed to render template: {}", e);
            ServerError::InternalError
        })?;

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    tag: Option<String>,
    page: Option<i64>,
}

pub async fn search(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, ServerError> {
    let page = query.page.unwrap_or(1);
    let mut context = Context::new();

    let conn = db::get_conn(&pool)
        .map_err(|e| {
            log::error!("Database error: {}", e);
            ServerError::InternalError
        })?;
    let posts = if query.tag.is_some() {
        let search_tag = query.tag.as_ref().unwrap().to_string();
        context.insert("search_tag", &search_tag);

        web::block(move || db::select_n_posts_with_tag(&search_tag, 20, (page - 1) * 15, conn))
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
            })?
    } else {
        Vec::new()
    };

    let conn = db::get_conn(&pool)
        .map_err(|e| {
            log::error!("Database error: {}", e);
            ServerError::InternalError
        })?;
    let total_posts = web::block(move || db::count_total_posts(conn))
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
    let total_pages = total_posts / 15;

    context.insert("posts", &posts);
    context.insert("page", &page);
    context.insert("total_pages", &total_pages);

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
    let conn = db::get_conn(&pool)
        .map_err(|e| {
            log::error!("Database error: {}", e);
            ServerError::InternalError
        })?;
    let post = web::block(move || db::select_post_with_slug(&post_slug, conn))
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

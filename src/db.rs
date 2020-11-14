use std::ops::Deref;

use actix_web::{error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use derive_more::{Display};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use crate::models::{NewPost, Post};

#[derive(Debug, Display)]
pub enum DBError {
    #[display(fmt = "not found")]
    SelectError,
    #[display(fmt = "bad request")]
    InsertError,
    #[display(fmt = "bad request")]
    UpdateError,
    #[display(fmt = "bad request")]
    DeleteError,
    #[display(fmt = "internal error")]
    PoolError,
}

impl error::ResponseError for DBError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            DBError::SelectError => StatusCode::NOT_FOUND,
            DBError::InsertError => StatusCode::BAD_REQUEST,
            DBError::UpdateError => StatusCode::BAD_REQUEST,
            DBError::DeleteError => StatusCode::BAD_REQUEST,
            DBError::PoolError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, DBError> {
    pool.get().map_err(|_| DBError::PoolError)
}

pub fn select_last_n_posts(n: i64, pool: &PgPool) -> Result<Vec<Post>, DBError> {
    Post::last_n_published(n, get_conn(pool)?.deref())
        .map_err(|_| DBError::SelectError)
}

pub fn select_post_with_slug(slug: &str, pool: &PgPool) -> Result<Post, DBError> {
    Post::select_with_slug(slug, get_conn(pool)?.deref())
        .map_err(|_| DBError::SelectError)
}

pub fn create_post(title: &str, body: &str, published: bool, pool: &PgPool) -> Result<(), DBError> {
    let new_post = NewPost::new(title, body, published);
    Post::insert(new_post, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| DBError::InsertError)
}

pub fn publish_post(slug: &str, pool: &PgPool) -> Result<(), DBError> {
    Post::publish_with_slug(slug, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| DBError::UpdateError)
}

pub fn delete_post(slug: &str, pool: &PgPool) -> Result<(), DBError> {
    Post::delete_with_slug(slug, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| DBError::DeleteError)
}

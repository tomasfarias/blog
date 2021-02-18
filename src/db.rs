use std::ops::Deref;

use derive_more::{Display, Error};
use diesel::pg::PgConnection;
use diesel::result;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};

use crate::models::Post;

#[derive(Debug, Display, Error)]
pub enum DatabaseError {
    #[display(fmt = "Error in connection pool : {}", _0)]
    ConnectionPoolError(PoolError),
    #[display(fmt = "Query returned no results: {}", _0)]
    NotFound(result::Error),
    #[display(fmt = "Error executing query: {}", _0)]
    QueryError(result::Error),
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, DatabaseError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .map_err(|e| DatabaseError::ConnectionPoolError(e))
}

pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, DatabaseError> {
    pool.get()
        .map_err(|e| DatabaseError::ConnectionPoolError(e))
}

pub fn select_last_n_posts(n: i64, offset: i64, pool: &PgPool) -> Result<Vec<Post>, DatabaseError> {
    Post::last_n_published(n, offset, get_conn(pool)?.deref())
        .map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

pub fn select_post_with_slug(slug: &str, pool: &PgPool) -> Result<Post, DatabaseError> {
    Post::select_with_slug(slug, get_conn(pool)?.deref()).map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

/*
pub fn insert_new_post(title: &str, body: &str, published: bool, pool: &PgPool) -> Result<Post, DatabaseError> {
    let new_post = NewPost::new(title, body, published);
    new_post.insert(get_conn(pool)?.deref()).map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

pub fn publish_post(slug: &str, pool: &PgPool) -> Result<(), DatabaseError> {
    Post::publish_with_slug(slug, get_conn(pool)?.deref())
        .map(|_| ()).map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

pub fn delete_post(slug: &str, pool: &PgPool) -> Result<(), DatabaseError> {
    Post::delete_with_slug(slug, get_conn(pool)?.deref())
        .map(|_| ()).map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}
*/

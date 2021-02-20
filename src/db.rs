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

pub fn count_total_posts(conn: PgPooledConnection) -> Result<i64, DatabaseError> {
    Post::count_total(&conn)
        .map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

pub fn select_last_n_posts(n: i64, offset: i64, conn: PgPooledConnection) -> Result<Vec<Post>, DatabaseError> {
    Post::last_n_published(n, offset, &conn)
        .map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

pub fn select_post_with_slug(slug: &str, conn: PgPooledConnection) -> Result<Post, DatabaseError> {
    Post::select_with_slug(slug, &conn).map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

pub fn select_n_posts_with_tag(tag: &str, n: i64, offset: i64, conn: PgPooledConnection) -> Result<Vec<Post>, DatabaseError> {
    Post::select_n_posts_with_tag(tag, n, offset, &conn).map_err(|e| {
            match e {
                result::Error::NotFound => DatabaseError::NotFound(e),
                _ => DatabaseError::QueryError(e)
            }
        })
}

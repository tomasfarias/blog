use chrono::{Utc, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::dsl::count_star;
use diesel::prelude::*;
use serde::Serialize;

use crate::models::schema::{posts, tags, post_tags};

#[derive(Serialize, Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub introduction: Option<String>,
    pub tags: Vec<String>,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub slug: String,
    pub body: String,
    pub introduction: Option<String>,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published_at: Option<NaiveDateTime>,
}

impl Post {
    pub fn count_total(conn: &PgConnection) -> QueryResult<i64> {
        posts::table.select(count_star()).first(conn)
    }

    pub fn last_n_published(n: i64, offset: i64, conn: &PgConnection) -> QueryResult<Vec<Self>> {
        posts::table.filter(posts::published.eq(true))
            .order(posts::published_at.desc())
            .offset(offset)
            .limit(n)
            .load::<Post>(conn)
    }

    pub fn select_with_slug(_slug: &str, conn: &PgConnection) -> QueryResult<Self> {
        posts::table.filter(posts::slug.eq(_slug))
            .first::<Post>(conn)
    }

    pub fn publish_with_slug(_slug: &str, conn: &PgConnection) -> QueryResult<usize> {
        let now = Utc::now().naive_utc();
        diesel::update(posts::table)
            .filter(posts::slug.eq(_slug))
            .set((posts::published.eq(true), posts::published_at.eq(now)))
            .execute(conn)
    }

    pub fn delete_with_slug(_slug: &str, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(posts::table)
            .filter(posts::slug.eq(_slug))
            .execute(conn)
    }

    pub fn select_n_posts_with_tag(_tag: &str, n:i64, offset: i64, conn: &PgConnection) -> QueryResult<Vec<Self>> {
        tags::table
            .filter(tags::name.eq(_tag))
            .inner_join(post_tags::table.inner_join(posts::table))
            .select((
                posts::id,
                posts::title,
                posts::slug,
                posts::body,
                posts::introduction,
                posts::tags,
                posts::published,
                posts::created_at,
                posts::updated_at,
                posts::published_at,
            ))
            .order(posts::published_at.desc())
            .offset(offset)
            .limit(n)
            .load::<Post>(conn)
    }
}

fn title_to_slug(title: &str) -> String {
    title.to_lowercase()
        .replace(" ", "-")
        .to_owned()
}

impl NewPost {
    pub fn new(title: &str, body: &str, introduction: Option<String>, published: bool) -> NewPost {
        let created_at = Utc::now().naive_utc();
        let published_at = match published {
            true => Some(Utc::now().naive_utc()),
            false => None,
        };

        NewPost {
            title: title.to_owned(),
            slug: title_to_slug(&title),
            body: body.to_owned(),
            introduction: introduction,
            published: published,
            created_at: created_at,
            updated_at: created_at.clone(),
            published_at: published_at,
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> QueryResult<Post> {
        diesel::insert_into(posts::table)
            .values(self)
            .get_result(conn)
    }
}

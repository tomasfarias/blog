use chrono::{Utc, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Serialize;

use crate::models::schema::users;


#[derive(Serialize, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn select_with_email(_email: &str, conn: &PgConnection) -> QueryResult<Self> {
        users::table.filter(users::email.eq(_email))
            .first::<User>(conn)
    }
}

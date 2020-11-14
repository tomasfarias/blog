table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        slug -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
    }
}

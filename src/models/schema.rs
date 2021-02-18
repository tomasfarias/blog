table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        body -> Text,
        introduction -> Nullable<Text>,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
    }
}

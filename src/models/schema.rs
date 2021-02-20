table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        slug -> Varchar,
        body -> Text,
        introduction -> Nullable<Text>,
        tags -> Array<Varchar>,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Varchar,
    }
}


table! {
    post_tags (id) {
        id -> Integer,
        post_id -> Integer,
        tag_id -> Integer,
    }
}

joinable!(post_tags -> posts (post_id));
joinable!(post_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    posts,
    tags,
    post_tags,
);

table! {
    comments (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    followers (follower_id, followee_id) {
        follower_id -> Int4,
        followee_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    followers,
    posts,
    users,
);

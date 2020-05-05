table! {
    episode (id) {
        id -> Integer,
        title -> Text,
        url -> Text,
        podcast_id -> Integer,
    }
}

table! {
    podcast (id) {
        id -> Integer,
        title -> Text,
        url -> Text,
    }
}

joinable!(episode -> podcast (podcast_id));

allow_tables_to_appear_in_same_query!(
    episode,
    podcast,
);

table! {
    config (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    points (id) {
        id -> Text,
        user_points -> Integer,
    }
}

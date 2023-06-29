// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        content -> Text,
        date -> Date,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

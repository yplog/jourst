// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        content -> Text,
        completed -> Bool,
        when_will_it_be_done -> Date,
    }
}

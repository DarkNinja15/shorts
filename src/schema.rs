// @generated automatically by Diesel CLI.

diesel::table! {
    shorts (id) {
        id -> Text,
        ref_url -> Text,
        title -> Text,
        description -> Text,
        author -> Text,
    }
}

diesel::table! {
    users (email) {
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(shorts -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    shorts,
    users,
);

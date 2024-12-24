// @generated automatically by Diesel CLI.

diesel::table! {
    labels (id) {
        id -> Integer,
        label -> Text,
        abb_ctx -> Text,
    }
}

diesel::table! {
    raws (id) {
        id -> Integer,
        date -> Date,
        ctx -> Text,
        amount -> Double,
        label -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(labels, raws,);

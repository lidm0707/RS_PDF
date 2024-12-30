// @generated automatically by Diesel CLI.

diesel::table! {
    credits (id) {
        id -> Integer,
        date -> Date,
        ctx -> Text,
        amount -> Double,
        label -> Text,
    }
}

diesel::table! {
    labels (id) {
        id -> Integer,
        id_label -> Integer,
        abb_ctx -> Text,
    }
}

diesel::table! {
    labels_name (id) {
        id -> Integer,
        label -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    credits,
    labels,
    labels_name,
);

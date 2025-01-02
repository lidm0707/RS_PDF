// @generated automatically by Diesel CLI.

diesel::table! {
    cash (id) {
        id -> Integer,
        date -> Date,
        period -> Text,
        label_id -> Integer,
        amount -> Double,
    }
}

diesel::table! {
    credits (id) {
        id -> Integer,
        date -> Date,
        ctx -> Text,
        amount -> Double,
        label_id -> Integer,
        period -> Text,
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

diesel::table! {
    setting_hotkey (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        label_id -> Integer,
        amount -> Double,
    }
}

diesel::table! {
    setting_pass_pdf (id) {
        id -> Integer,
        ctx -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    t1 (id) {
        id -> Integer,
        date_stard -> Date,
        date_end -> Date,
        time -> Integer,
        label_id -> Integer,
        amount -> Double,
        total -> Double,
    }
}

diesel::table! {
    t1_items (id) {
        id -> Integer,
        date -> Date,
        period -> Text,
        t1_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cash,
    credits,
    labels,
    labels_name,
    setting_hotkey,
    setting_pass_pdf,
    t1,
    t1_items,
);

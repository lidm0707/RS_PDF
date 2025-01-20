// @generated automatically by Diesel CLI.

diesel::table! {
    bank (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    cash (id) {
        id -> Integer,
        date -> Date,
        period -> Text,
        type_cash -> Text,
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
        payment_type_id -> Integer,
    }
}

diesel::table! {
    installment (id) {
        id -> Integer,
        date_start -> Date,
        date_end -> Date,
        time -> Integer,
        note -> Text,
        label_id -> Integer,
        amount -> Double,
        total -> Double,
    }
}

diesel::table! {
    installment_items (id) {
        id -> Integer,
        date -> Date,
        period -> Text,
        bank_id -> Integer,
        amount -> Double,
        installment_id -> Integer,
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
        ord -> Integer,
        show_able -> Bool,
    }
}

diesel::table! {
    payment_type (id) {
        id -> Integer,
        chanel -> Text,
    }
}

diesel::table! {
    planing_cash (id) {
        id -> Integer,
        period -> Text,
        label_id -> Integer,
        amount -> Double,
    }
}

diesel::table! {
    planing_credit (id) {
        id -> Integer,
        period -> Text,
        label_id -> Integer,
        amount -> Double,
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

diesel::allow_tables_to_appear_in_same_query!(
    bank,
    cash,
    credits,
    installment,
    installment_items,
    labels,
    labels_name,
    payment_type,
    planing_cash,
    planing_credit,
    setting_hotkey,
    setting_pass_pdf,
);

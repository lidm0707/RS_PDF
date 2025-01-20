use crate::backend::repo::schema::*;
use diesel::{allow_columns_to_appear_in_same_group_by_clause, joinable};
// use diesel::prelude::*;

joinable!(installment_items-> installment (installment_id));
allow_columns_to_appear_in_same_group_by_clause!(
    installment_items::period,
    installment_items::installment_id,
    installment::label_id,
);

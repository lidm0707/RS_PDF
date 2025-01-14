use crate::backend::{
    entity::entity_credit::SelectCredit, repo::db_credit::db_insert::insert_credit,
};

pub fn set_credit(
    date: String,
    ctx: String,
    amount: f64,
    label_id: i32,
    period: String,
    payment_type_id: i32,
) -> SelectCredit {
    let result = insert_credit(date, ctx, amount, label_id, period, payment_type_id);
    result
}

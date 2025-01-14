
use crate::backend::{model::model_credit::ModelCredit, repo::db_credit::db_insert::insert_credit};

pub fn sv_set_credit(
    date: String,
    ctx: String,
    amount: f64,
    label_id: i32,
    period: String,
    payment_type_id: i32,
) -> ModelCredit {
    let raw_data = insert_credit(date, ctx, amount, label_id, period, payment_type_id);
    ModelCredit {
        id: raw_data.id,
        date: raw_data.date,
        ctx: raw_data.ctx,
        amount: raw_data.amount,
        label_id: raw_data.label_id,
        period: raw_data.period,
        payment_type_id: raw_data.payment_type_id,
    }
}

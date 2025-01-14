use crate::backend::{
    model::model_credit::ModelCredit, service::sv_data::sv_set_credit::sv_set_credit,
};

pub fn set_credit(
    date: String,
    ctx: String,
    amount: f64,
    label_id: i32,
    period: String,
    payment_type_id: i32,
) -> ModelCredit {
    let result = sv_set_credit(date, ctx, amount, label_id, period, payment_type_id);
    result
}

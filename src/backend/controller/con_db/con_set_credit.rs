use crate::backend::{
    model::{model_credit::ModelCredit, model_pdf::TranformLine},
    service::sv_data::sv_set_credit::{sv_set_credit, sv_set_credit_bacth},
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

pub fn set_credit_bacth(data: Vec<TranformLine>) -> Vec<ModelCredit> {
    for i in data.iter() {
        if i.label_id == 0 || i.payment_type_id == 0 {
            return Vec::new();
        }
    }
    let result = sv_set_credit_bacth(data);
    result
}

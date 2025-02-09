
use crate::backend::{entity::entity_credit::InsertCredit, model::{model_credit::ModelCredit, model_pdf::TranformLine}, repo::db_credit::db_insert::{insert_credit, insert_credit_bacth}};

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


pub fn sv_set_credit_bacth(
    data: Vec<TranformLine>,
) -> Vec<ModelCredit> {
    // Transform `TranformLine` into `InsertCredit`
    let insert_credits: Vec<InsertCredit> = data
        .iter()
        .map(|td| InsertCredit {
            date: td.date.to_string(),
            ctx: td.ctx.to_string(),
            amount: td.amount,
            label_id: td.label_id as i32,
            period: td.period.to_string(),
            payment_type_id: td.payment_type_id as i32,
        })
        .collect();

    // Insert the `InsertCredit` records into the database
    let inserted_credits = insert_credit_bacth(insert_credits).unwrap();

    // If needed, transform `InsertCredit` into `ModelCredit`
    let model_credits: Vec<ModelCredit> = inserted_credits
        .into_iter()
        .map(|ic| ModelCredit {
            id: ic.id,
            date: ic.date,
            ctx: ic.ctx,
            amount: ic.amount,
            label_id: ic.label_id,
            period: ic.period,
            payment_type_id: ic.payment_type_id,
        })
        .collect();

    // Return the `ModelCredit` records
    model_credits
}

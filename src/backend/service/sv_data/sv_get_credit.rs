use crate::backend::{model::model_credit::ModelCredit, repo::db_credit::db_select::select_credit};

pub fn sv_get_credit() -> Result<Vec<ModelCredit>, anyhow::Error> {
    let raw_data = select_credit().unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelCredit {
            id: raw.id,
            date: raw.date,
            ctx: raw.ctx,
            amount: raw.amount,
            label_id: raw.label_id,
            period: raw.period,
            payment_type_id: raw.payment_type_id,
        })
        .collect();
    Ok(result)
}

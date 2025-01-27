use crate::backend::{model::model_cash_in::ModelCashIn, service::sv_data::sv_set_cash_in::sv_set_cash_in};

pub fn set_cash_in(
    date_value: String,
    period_value: String,
    revenue_id_value: i32,
    note_value:Option<String>,
    amount_value: f64,
) -> Result<ModelCashIn, anyhow::Error> {
    let result = sv_set_cash_in(
        date_value,
        period_value,
        revenue_id_value,
        note_value,
        amount_value,
    )
    .unwrap();
    Ok(result)
}

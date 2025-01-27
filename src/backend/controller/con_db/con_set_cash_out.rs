use crate::backend::{model::model_cash_out::ModelCashOut, service::sv_data::sv_set_cash_out::sv_set_cash_out};

pub fn set_cash_out(
    date_value: String,
    period_value: String,
    label_id_value: i32,
    note_value:Option<String>,
    amount_value: f64,
) -> Result<ModelCashOut, anyhow::Error> {
    let result = sv_set_cash_out(
        date_value,
        period_value,
        label_id_value,
        note_value,
        amount_value,
    )
    .unwrap();
    Ok(result)
}

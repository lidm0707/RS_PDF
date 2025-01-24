use crate::backend::{model::model_cash::ModelCash, service::sv_data::sv_set_cash::sv_set_cash};

pub fn set_cash(
    date_value: String,
    period_value: String,
    type_cash_value: String,
    label_id_value: i32,
    note_value:Option<String>,
    amount_value: f64,
) -> Result<ModelCash, anyhow::Error> {
    let result = sv_set_cash(
        date_value,
        period_value,
        type_cash_value,
        label_id_value,
        note_value,
        amount_value,
    )
    .unwrap();
    Ok(result)
}

use crate::backend::{model::model_cash::ModelCash, repo::db_cash::db_insert::insert_cash};

pub fn sv_set_cash(
    date_value: String,
    period_value: String,
    type_cash_value: String,
    label_id_value: i32,
    note_value: Option<String>,
    amount_value: f64,
) -> Result<ModelCash, anyhow::Error> {
    let raw_data = insert_cash(
        date_value,
        period_value,
        type_cash_value,
        label_id_value,
        note_value,
        amount_value,
    );

    let result = ModelCash {
        id: raw_data.id,
        date: raw_data.date,
        period: raw_data.period,
        type_cash: raw_data.type_cash,
        label_id: raw_data.label_id,
        note:raw_data.note,
        amount: raw_data.amount,
    };

    Ok(result)
}

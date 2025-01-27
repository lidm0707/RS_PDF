use crate::backend::{model::model_cash_out::ModelCashOut, repo::db_cash_out::db_insert::insert_cash_out};

pub fn sv_set_cash_out(
    date_value: String,
    period_value: String,
    label_id_value: i32,
    note_value: Option<String>,
    amount_value: f64,
) -> Result<ModelCashOut, anyhow::Error> {
    let raw_data = insert_cash_out(
        date_value,
        period_value,
        label_id_value,
        note_value,
        amount_value,
    );

    let result = ModelCashOut {
        id: raw_data.id,
        date: raw_data.date,
        period: raw_data.period,
        label_id: raw_data.label_id,
        note:raw_data.note,
        amount: raw_data.amount,
    };

    Ok(result)
}

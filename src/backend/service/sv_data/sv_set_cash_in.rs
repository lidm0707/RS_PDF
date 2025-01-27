use crate::backend::{model::model_cash_in::ModelCashIn, repo::db_cash_in::db_insert::insert_cash_in};

pub fn sv_set_cash_in(
    date_value: String,
    period_value: String,
    revenue_id_value: i32,
    note_value: Option<String>,
    amount_value: f64,
) -> Result<ModelCashIn, anyhow::Error> {
    let raw_data = insert_cash_in(
        date_value,
        period_value,
        revenue_id_value,
        note_value,
        amount_value,
    );

    let result = ModelCashIn {
        id: raw_data.id,
        date: raw_data.date,
        period: raw_data.period,
        revenue_id: raw_data.revenue_id,
        note:raw_data.note,
        amount: raw_data.amount,
    };

    Ok(result)
}

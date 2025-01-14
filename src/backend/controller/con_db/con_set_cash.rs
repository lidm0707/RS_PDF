use crate::backend::{entity::entity_cash::SelectCash, repo::db_cash::db_insert::insert_cash};

pub fn set_cash(
    date_value: String,
    period_value: String,
    type_cash_value: String,
    label_id_value: i32,
    amount_value: f64,
) -> Result<SelectCash, anyhow::Error> {
    let result = insert_cash(
        date_value,
        period_value,
        type_cash_value,
        label_id_value,
        amount_value,
    );
    Ok(result)
}

use crate::backend::{model::model_cash_out::ModelCashOut, service::sv_data::sv_get_cash_out::sv_get_cash_out};

pub fn get_cash_out() -> Result<Vec<ModelCashOut>, anyhow::Error> {
    let result = sv_get_cash_out().unwrap();

    Ok(result)
}

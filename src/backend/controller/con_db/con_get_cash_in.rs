use crate::backend::{model::model_cash_in::ModelCashIn, service::sv_data::sv_get_cash_in::sv_get_cash_in};

pub fn get_cash_in() -> Result<Vec<ModelCashIn>, anyhow::Error> {
    let result = sv_get_cash_in().unwrap();

    Ok(result)
}
    
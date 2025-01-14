use crate::backend::{model::model_cash::ModelCash, service::sv_data::sv_get_cash::sv_get_cash};

pub fn get_cash() -> Result<Vec<ModelCash>, anyhow::Error> {
    let result = sv_get_cash().unwrap();

    Ok(result)
}

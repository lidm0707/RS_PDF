use crate::backend::{model::model_credit::ModelCredit, service::sv_data::sv_get_credit::sv_get_credit};

pub fn get_credit() -> Result<Vec<ModelCredit>, anyhow::Error> {
    let result = sv_get_credit().unwrap();

    Ok(result)
}

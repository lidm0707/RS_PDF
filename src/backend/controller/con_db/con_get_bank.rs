use crate::backend::{
    model::model_bank::ModelBank,
    service::sv_data::sv_get_bank::{sv_get_bank, sv_get_bank_where},
};

pub fn get_bank() -> Result<Vec<ModelBank>, anyhow::Error> {
    let result = sv_get_bank().unwrap();
    Ok(result)
}

pub fn get_bank_where(input: i32) -> Result<Vec<ModelBank>, anyhow::Error> {
    let result = sv_get_bank_where(input).unwrap();
    Ok(result)
}

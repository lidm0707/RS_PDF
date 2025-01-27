use crate::backend::{
    model::model_payment_type::ModelPaymentType,
     service::sv_data::sv_get_payment::{sv_get_payment_type, sv_get_payment_type_where},
};

pub fn get_payment_type_where(input_id: i32) -> Result<Vec<ModelPaymentType>, anyhow::Error> {
    // Safely call `select_payment_type_where` and propagate errors
    let result = sv_get_payment_type_where(input_id)?;

    Ok(result)
}


pub fn get_payment_type() -> Result<Vec<ModelPaymentType>, anyhow::Error> {
    // Safely call `select_payment_type_where` and propagate errors
    let result = sv_get_payment_type()?;

    Ok(result)
}


use crate::backend::{
    model::model_installment::{ModelInstallment, ModelInstallmentItems},
    service::sv_data::sv_get_installment::{
        sv_get_installment, sv_get_installment_items, sv_get_installment_items_where,
    },
};

pub fn get_installment() -> Result<Vec<ModelInstallment>, anyhow::Error> {
    let result = sv_get_installment().unwrap();

    Ok(result)
}

pub fn get_installment_items() -> Result<Vec<ModelInstallmentItems>, anyhow::Error> {
    let result = sv_get_installment_items().unwrap();

    Ok(result)
}

pub fn get_installment_items_where(
    input_id: i32,
) -> Result<Vec<ModelInstallmentItems>, anyhow::Error> {
    let result = sv_get_installment_items_where(input_id).unwrap();

    Ok(result)
}

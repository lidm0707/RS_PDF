use crate::backend::{
    model::model_payment_type::ModelPaymentType,
    repo::db_payment::db_select::select_payment_type_where,
};

pub fn sv_get_payment_type_where(input_id: i32) -> Result<Vec<ModelPaymentType>, anyhow::Error> {
    // Safely call `select_payment_type_where` and propagate errors
    let raw = select_payment_type_where(input_id)?;

    // Map `SelectPaymentType` to `ModelPaymentType`
    let result = raw
        .into_iter()
        .map(|data| ModelPaymentType {
            id: data.id,
            chanel: data.chanel,
        })
        .collect();

    Ok(result)
}

use crate::backend::{
    model::model_installment::{ModelInstallment, ModelInstallmentItems},
    repo::db_installment::db_select::{
        select_installment, select_installment_items, select_installment_items_where,
    },
};

pub fn get_installment() -> Result<Vec<ModelInstallment>, anyhow::Error> {
    let raw_data = select_installment().unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelInstallment {
            id: raw.id,
            date_start: raw.date_start,
            date_end: raw.date_end,
            time: raw.time,
            note: raw.note,
            label_id: raw.label_id,
            amount: raw.amount,
            total: raw.total,
        })
        .collect();

    Ok(result)
}

pub fn get_installment_items() -> Result<Vec<ModelInstallmentItems>, anyhow::Error> {
    let raw_data = select_installment_items().unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelInstallmentItems {
            id: raw.id,
            date: raw.date,
            period: raw.period,
            bank_id: raw.bank_id,
            amount: raw.amount,
            installment_id: raw.installment_id,
        })
        .collect();
    Ok(result)
}

pub fn get_installment_items_where(
    input_id: i32,
) -> Result<Vec<ModelInstallmentItems>, anyhow::Error> {
    let raw_data = select_installment_items_where(input_id).unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelInstallmentItems {
            id: raw.id,
            date: raw.date,
            period: raw.period,
            bank_id: raw.bank_id,
            amount: raw.amount,
            installment_id: raw.installment_id,
        })
        .collect();
    Ok(result)
}

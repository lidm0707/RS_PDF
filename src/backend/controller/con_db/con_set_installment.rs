use crate::backend::{
    model::model_installment::{ModelInstallment, ModelInstallmentItems},
    service::sv_data::sv_set_installment::{sv_set_installment, sv_set_installment_items},
};

pub fn set_installment(
    date_stard_value: String,
    date_end_value: String,
    time_value: i32,
    note_value: String,
    label_id_value: i32,
    amount_value: f64,
    total_value: f64,
) -> ModelInstallment {
    let result = sv_set_installment(
        date_stard_value,
        date_end_value,
        time_value,
        note_value,
        label_id_value,
        amount_value,
        total_value,
    );

    result
}

pub fn set_installment_items(
    date_value: String,
    period_value: String,
    bank_id_value: i32,
    amount_value: f64,
    installment_id_value: i32,
) -> ModelInstallmentItems {
    let result = sv_set_installment_items(
        date_value,
        period_value,
        bank_id_value,
        amount_value,
        installment_id_value,
    );
    result
}

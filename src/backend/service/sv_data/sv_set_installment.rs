use crate::backend::{
    model::model_installment::{ModelInstallment, ModelInstallmentItems},
    repo::db_installment::db_insert::{insert_installment, insert_installment_items},
};

pub fn sv_set_installment(
    date_start_value: String,
    date_end_value: String,
    time_value: i32,
    note_value: String,
    label_id_value: i32,
    amount_value: f64,
    total_value: f64,
) -> ModelInstallment {
    let result = insert_installment(
        date_start_value,
        date_end_value,
        time_value,
        note_value,
        label_id_value,
        amount_value,
        total_value,
    );

    ModelInstallment {
        id:result.id,
        date_start:result.date_start,
        date_end:result.date_end,
        time:result.time,
        note:result.note,
        label_id:result.label_id,
        amount:result.amount,
        total:result.total,
    }
}

pub fn sv_set_installment_items(
    date_value: String,
    period_value: String,
    bank_id_value: i32,
    amount_value: f64,
    installment_id_value: i32,
) -> ModelInstallmentItems {
    let result = insert_installment_items(
        date_value,
        period_value,
        bank_id_value,
        amount_value,
        installment_id_value,
    );
    ModelInstallmentItems {
        id: result.id,
        date: result.date,
        period: result.period,
        bank_id: result.bank_id,
        amount: result.amount,
        installment_id: result.installment_id,
    }
}

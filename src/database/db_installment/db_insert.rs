use crate::database::db_connect::connect_database;
use crate::database::schema::installment::dsl::*;
use crate::database::schema::installment_items::dsl::*;
use crate::entity::entity_installment::*;
use diesel::prelude::*;

pub fn insert_installment(
    date_stard_value: String,
    date_end_value: String,
    time_value: i32,
    note_value: String,
    label_id_value: i32,
    amount_value: f64,
    total_value: f64,
) -> SelectInstallment {
    let mut conn = connect_database();

    let new_post = InsertInstallment {
        date_stard: date_stard_value,
        date_end: date_end_value,
        time: time_value,
        note: note_value,
        label_id: label_id_value,
        amount: amount_value,
        total: total_value,
    };

    diesel::insert_into(installment)
        .values(new_post)
        .returning(SelectInstallment::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn insert_installment_items(
    date_value: String,
    period_value: String,
    bank_id_value: i32,
    amount_value: f64,
    installment_id_value: i32,
) -> SelectInstallmentItems {
    let mut conn = connect_database();

    let new_post = InsertInstallmentItems {
        date: date_value,
        period: period_value,
        bank_id:bank_id_value,
        amount:amount_value,
        installment_id: installment_id_value,
    };



    diesel::insert_into(installment_items)
        .values(new_post)
        .returning(SelectInstallmentItems::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

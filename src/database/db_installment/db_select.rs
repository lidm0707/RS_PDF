use self::database::schema::installment::dsl::*;
use self::database::schema::installment_items::dsl::*;
use crate::entity::entity_installment::*;
use crate::*;
use anyhow;
use database::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_installment() -> Result<Vec<SelectInstallment>, anyhow::Error> {
    let mut conn = connect_database();

    let results = installment
        .select(SelectInstallment::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}

pub fn select_installment_items() -> Result<Vec<SelectInstallmentItems>, anyhow::Error> {
    let mut conn = connect_database();

    let results = installment_items
        .select(SelectInstallmentItems::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}
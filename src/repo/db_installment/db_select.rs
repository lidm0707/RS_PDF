use self::repo::schema::installment::dsl::*;
use self::repo::schema::installment_items::dsl::{*, id as installment_items_id};
use crate::entity::entity_installment::*;
use crate::*;
use anyhow;
use repo::db_connect::connect_database;
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

pub fn select_installment_items_where(
    input_id: i32,
) -> Result<Vec<SelectInstallmentItems>, anyhow::Error> {
    let mut conn = connect_database();

    let results = installment_items
        .filter(installment_id.eq(input_id))
        .select(SelectInstallmentItems::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}

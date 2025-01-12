use self::repo::schema::installment::dsl::*;
use self::repo::schema::installment_items::dsl::*;
use crate::entity::entity_installment::*;
use crate::*;
use anyhow;
use diesel::dsl::sum;
use diesel::prelude::*;

use repo::db_connect::connect_database;
use service::date::date_format::format_period;
use service::date::now::thai_now;

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




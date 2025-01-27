
use self::backend::repo::schema::payment_type::dsl::*;
use crate::backend::entity::entity_payment::*;
use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_payment_type_where(input_id: i32) -> Result<Vec<SelectPaymentType>, anyhow::Error> {
    let mut conn = connect_database();

    let result = payment_type
        .filter(id.eq(input_id))
        .select(SelectPaymentType::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}



pub fn select_payment_type() -> Result<Vec<SelectPaymentType>, anyhow::Error> {
    let mut conn = connect_database();

    let result = payment_type
        .select(SelectPaymentType::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}
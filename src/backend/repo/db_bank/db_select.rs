use self::backend::repo::schema::bank::dsl::*;
use crate::backend::entity::entity_bank::*;
use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_bank() -> Result<Vec<SelectBank>, anyhow::Error> {
    let mut conn = connect_database();

    let results = bank
        .select(SelectBank::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}

pub fn select_bank_where(input_id: i32) -> Result<Vec<SelectBank>, anyhow::Error> {
    let mut conn = connect_database();

    let results = bank
        .filter(id.eq(input_id))
        .select(SelectBank::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}


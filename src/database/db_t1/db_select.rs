use self::database::schema::t1::dsl::*;
use self::database::schema::t1_items::dsl::*;
use crate::entity::entity_t1::*;
use crate::*;
use anyhow;
use database::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_t1() -> Result<Vec<SelectT1>, anyhow::Error> {
    let mut conn = connect_database();

    let results = t1
        .select(SelectT1::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}

pub fn select_t1_items() -> Result<Vec<SelectT1Items>, anyhow::Error> {
    let mut conn = connect_database();

    let results = t1_items
        .select(SelectT1Items::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}
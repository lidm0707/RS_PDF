
use self::database::schema::credits::dsl::*;
use crate::entity::entity_credit::*;
use crate::*;
use anyhow;
use database::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_credit() -> Result<Vec<SelectCredit>, anyhow::Error> {
    let mut conn = connect_database();

    let results = credits
        .select(SelectCredit::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}
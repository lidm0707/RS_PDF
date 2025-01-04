use self::database::schema::cash::dsl::*;
use crate::entity::entity_cash::*;
use crate::*;
use anyhow;
use database::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_cash() -> Result<Vec<SelectCash>, anyhow::Error> {
    let mut conn = connect_database();

    let results = cash
        .select(SelectCash::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}




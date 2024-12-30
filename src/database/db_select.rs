use crate::models::{model_label::*, model_raw::*};
use crate::*;
use anyhow;
use database::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_labels() -> Result<Vec<SelectLabels>, anyhow::Error> {
    use self::database::schema::labels::dsl::*;
    let mut conn = connect_database();

    let results = labels
        .select(SelectLabels::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

pub fn select_raws() -> Result<Vec<SelectRaws>, anyhow::Error> {
    use self::database::schema::raws::dsl::*;
    let mut conn = connect_database();

    let results = raws
        .select(SelectRaws::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

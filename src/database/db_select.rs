use crate::entity::{entity_credit::*, entity_label::*};
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

pub fn select_labels_where(input_id: i32) -> Result<Vec<SelectLabels>, anyhow::Error> {
    use self::database::schema::labels::dsl::*;
    let mut conn = connect_database();

    let results = labels
        .filter(id_label.eq(input_id))
        .select(SelectLabels::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}


pub fn count_labels_where(input_id: i32) -> Result<i64, anyhow::Error> {
    use self::database::schema::labels::dsl::*;
    let mut conn = connect_database();

    let result = labels
        .filter(id_label.eq(input_id))
        .count()
        .get_result(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}

pub fn select_labels_name() -> Result<Vec<SelectLabelsName>, anyhow::Error> {
    use self::database::schema::labels_name::dsl::*;
    let mut conn = connect_database();

    let results = labels_name
        .select(SelectLabelsName::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

pub fn select_raws() -> Result<Vec<SelectCredit>, anyhow::Error> {
    use self::database::schema::credits::dsl::*;
    let mut conn = connect_database();

    let results = credits
        .select(SelectCredit::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

use self::database::schema::credits::dsl::*;
use self::database::schema::labels::dsl::*;
use self::database::schema::labels_name::dsl::{*, id as labels_name_id};
use crate::entity::{entity_credit::*, entity_label::*};
use crate::*;
use anyhow;
use database::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_labels() -> Result<Vec<SelectLabels>, anyhow::Error> {
    let mut conn = connect_database();

    let results = labels
        .select(SelectLabels::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

pub fn select_labels_where(input_id: i32) -> Result<Vec<SelectLabels>, anyhow::Error> {
    let mut conn = connect_database();

    let results = labels
        .filter(id_label.eq(input_id))
        .select(SelectLabels::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

pub fn count_labels_where(input_id: i32) -> Result<i64, anyhow::Error> {
    let mut conn = connect_database();

    let result = labels
        .filter(id_label.eq(input_id))
        .count()
        .get_result(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}

pub fn select_labels_like(search_pattern: &str) -> Result<Vec<SelectLabels>, anyhow::Error> {
    let mut conn = connect_database();
    let pattern = format!("%{}%", search_pattern); // ใช้ % เพื่อ match pattern

    let result = labels
        .filter(abb_ctx.eq(&pattern))
        .select(SelectLabels::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}

pub fn select_labels_name() -> Result<Vec<SelectLabelsName>, anyhow::Error> {
    let mut conn = connect_database();

    let results = labels_name
        .select(SelectLabelsName::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

pub fn select_labels_name_where(input_id: i32) -> Result<Vec<SelectLabelsName>, anyhow::Error> {
    let mut conn = connect_database();

    let result = labels_name
        .filter(labels_name_id.eq(input_id))
        .select(SelectLabelsName::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}

pub fn select_credit() -> Result<Vec<SelectCredit>, anyhow::Error> {
    let mut conn = connect_database();

    let results = credits
        .select(SelectCredit::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

// pub fn select_labels_like(search_pattern: &str) -> Result<Vec<SelectLabels>, anyhow::Error> {
//     let mut conn = connect_database();
//     let pattern = format!("%{}%", search_pattern); // ใช้ % เพื่อ match pattern

//     let result = labels
//         .filter(abb_ctx.eq(&pattern))
//         .select(SelectLabels::as_select())
//         .load(&mut conn)
//         .expect("Error loading posts");

//     Ok(result)
// }

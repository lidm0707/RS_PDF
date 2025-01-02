use crate::database::db_connect::connect_database;
use crate::database::schema::t1::dsl::*;
use crate::database::schema::t1_items::dsl::*;
use crate::entity::entity_t1::*;
use diesel::prelude::*;

pub fn insert_t1(
    id_value: i32,
    date_stard_value: String,
    date_end_value: String,
    time_value: i32,
    label_id_value: i32,
    amount_value: f64,
    total_value: f64,
) -> SelectT1 {
    let mut conn = connect_database();

    let new_post = InsertT1 {
        date_stard: date_stard_value,
        date_end:date_end_value,
        time:time_value,
        label_id:label_id_value,
        amount:amount_value,
        total:total_value,
    };

    diesel::insert_into(t1)
        .values(new_post)
        .returning(SelectT1::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn insert_t1_items(
    date_value: String,
    period_value: String,
    t1_id_value: i32,
) -> SelectT1Items {
    let mut conn = connect_database();

    let new_post = InsertT1Items {
        date: date_value,
        period: period_value,
        t1_id: t1_id_value,
    };

    diesel::insert_into(t1_items)
        .values(new_post)
        .returning(SelectT1Items::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

use self::backend::repo::schema::*;
use crate::backend::entity::entity_revenue_type::*;

use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_revenue_type() -> Result<Vec<SelectRevenueType>, anyhow::Error> {
    //payment_type:i32
    let mut conn: SqliteConnection = connect_database();
    //.and(payment_type_id.eq(payment_type))
    let results = revenue_type::table
        .select(SelectRevenueType::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}


pub fn select_revenue_type_where(input_id:i32) -> Result<Vec<SelectRevenueType>, anyhow::Error> {
    //payment_type:i32
    let mut conn: SqliteConnection = connect_database();
    //.and(payment_type_id.eq(payment_type))
    let results = revenue_type::table
        .filter(revenue_type::id.eq(input_id))
        .select(SelectRevenueType::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}

use crate::backend::entity::entity_revenue_type::*;
use crate::backend::repo::db_connect::connect_database;
use crate::backend::repo::schema::revenue_type;
use diesel::prelude::*;

pub fn insert_revenue_type(category: String) -> SelectRevenueType {
    let mut conn = connect_database();

    let new_post = InsertRevenueType { category };

    diesel::insert_into(revenue_type::table)
        .values(new_post)
        .returning(SelectRevenueType::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

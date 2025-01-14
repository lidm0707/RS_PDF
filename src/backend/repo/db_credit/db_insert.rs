use crate::{
    backend::repo::db_connect::connect_database,
    backend::entity::entity_credit::*,
};
use diesel::prelude::*;


pub fn insert_credit(
    date: String,
    ctx: String,
    amount: f64,
    label_id: i32,
    period: String,
    payment_type_id: i32,
) -> SelectCredit {
    use crate::backend::repo::schema::credits;
    let new_post = InsertCredit {
        date,
        ctx,
        amount,
        label_id,
        period,
        payment_type_id,
    };
    let mut conn = connect_database();

    diesel::insert_into(credits::table)
        .values(new_post)
        .returning(SelectCredit::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

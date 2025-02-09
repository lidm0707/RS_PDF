use crate::{
    backend::entity::entity_credit::*, backend::repo::db_connect::connect_database,
    backend::repo::schema::credits,
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

pub fn insert_credit_bacth(data: Vec<InsertCredit>) -> Result<Vec<SelectCredit>, anyhow::Error> {
    let mut conn = connect_database();

    // Insert the batch of credits
    diesel::insert_into(credits::table)
        .values(&data)
        .execute(&mut conn)?;

    // Fetch the inserted records
    let inserted_credits = credits::table
        .order(credits::id.desc())
        .limit(data.len() as i64)
        .load::<SelectCredit>(&mut conn)?;

    Ok(inserted_credits)
}

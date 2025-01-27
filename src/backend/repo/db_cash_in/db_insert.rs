use crate::backend::repo::db_connect::connect_database;
use crate::backend::repo::schema::cash_in::dsl::*;
use crate::backend::entity::entity_cash_in::*;
use diesel::prelude::*;


pub fn insert_cash_in(
    date_value: String,
    period_value: String,
    revenue_id_value: i32,
    note_value:Option<String> ,
    amount_value: f64,
) -> SelectCashIn {
    let mut conn = connect_database();

    let new_post = InsertCashIn {
        date: date_value,
        period: period_value,
        revenue_id: revenue_id_value,
        note:note_value,
        amount: amount_value,
 
    };

    diesel::insert_into(cash_in)
        .values(new_post)
        .returning(SelectCashIn::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

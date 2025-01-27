use crate::backend::repo::db_connect::connect_database;
use crate::backend::repo::schema::cash_out::dsl::*;
use crate::backend::entity::entity_cash_out::*;
use diesel::prelude::*;


pub fn insert_cash_out(
    date_value: String,
    period_value: String,
    label_id_value: i32,
    note_value:Option<String> ,
    amount_value: f64,
) -> SelectCashOut {
    let mut conn = connect_database();

    let new_post = InsertCashOut {
        date: date_value,
        period: period_value,
        label_id: label_id_value,
        note:note_value,
        amount: amount_value,
 
    };

    diesel::insert_into(cash_out)
        .values(new_post)
        .returning(SelectCashOut::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

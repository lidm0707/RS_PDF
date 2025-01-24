use crate::backend::repo::db_connect::connect_database;
use crate::backend::repo::schema::cash::dsl::*;
use crate::backend::entity::entity_cash::*;
use diesel::prelude::*;


pub fn insert_cash(
    date_value: String,
    period_value: String,
    type_cash_value: String,
    label_id_value: i32,
    note_value:Option<String> ,
    amount_value: f64,
) -> SelectCash {
    let mut conn = connect_database();

    let new_post = InsertCash {
        date: date_value,
        period: period_value,
        type_cash: type_cash_value,
        label_id: label_id_value,
        note:note_value,
        amount: amount_value,
 
    };

    diesel::insert_into(cash)
        .values(new_post)
        .returning(SelectCash::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

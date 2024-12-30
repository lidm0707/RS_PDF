use crate::{
    database::db_connect::connect_database,
    entity::{entity_credit::*, entity_label::*},
};
use diesel::{RunQueryDsl, SelectableHelper};

pub fn insert_label(id_label: i32, abb_ctx: String) -> SelectLabels {
    use crate::database::schema::labels;
    let mut conn = connect_database();

    let new_post = InsertLabels { id_label, abb_ctx };

    diesel::insert_into(labels::table)
        .values(new_post)
        .returning(SelectLabels::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn insert_credit(date: String, ctx: String, amount: f64, label: String) -> SelectCredit {
    use crate::database::schema::credits;
    let new_post = InsertCredit {
        date,
        ctx,
        amount,
        label,
    };
    let mut conn = connect_database();

    diesel::insert_into(credits::table)
        .values(new_post)
        .returning(SelectCredit::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

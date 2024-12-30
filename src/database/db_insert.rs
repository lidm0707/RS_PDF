use crate::{
    database::db_connect::connect_database,
    entity::{entity_credit::*, entity_label::*},
};
use diesel::{RunQueryDsl, SelectableHelper, SqliteConnection};

pub fn insert_label(label: String, abb_ctx: String) -> SelectLabels {
    use crate::database::schema::labels;
    let mut conn = connect_database();

    let new_post = InsertLabels { label, abb_ctx };

    diesel::insert_into(labels::table)
        .values(new_post)
        .returning(SelectLabels::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn insert_credit(
    conn: &mut SqliteConnection,
    date: String,
    ctx: String,
    amount: f64,
    label: String,
) -> SelectCredit {
    use crate::database::schema::raws;
    let new_post = InsertCredit {
        date,
        ctx,
        amount,
        label,
    };

    diesel::insert_into(raws::table)
        .values(&new_post)
        .returning(SelectCredit::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

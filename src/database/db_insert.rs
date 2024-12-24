use crate::models::{model_label::*, model_raw::*};
use diesel::{RunQueryDsl, SelectableHelper, SqliteConnection};

pub fn insert_labels(conn: &mut SqliteConnection, label: String, amount: f64) -> SelectLabels {
    use crate::database::schema::labels;

    let new_post = InsertLabels {
        label,
        abb_ctx: amount.to_string(),
    };

    diesel::insert_into(labels::table)
        .values(new_post)
        .returning(SelectLabels::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn insert_raws(
    conn: &mut SqliteConnection,
    date: String,
    ctx: String,
    amount: f64,
    label: String,
) -> SelectRaws {
    use crate::database::schema::raws;
    let new_post = InsertRaws {
        date,
        ctx,
        amount,
        label,
    };

    diesel::insert_into(raws::table)
        .values(&new_post)
        .returning(SelectRaws::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

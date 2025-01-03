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

pub fn insert_label_name(label: String) -> SelectLabelsName {
    use crate::database::schema::labels_name;
    let mut conn = connect_database();

    let new_post = InsertLabelsName { label };

    diesel::insert_into(labels_name::table)
        .values(new_post)
        .returning(SelectLabelsName::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn insert_credit(
    date: String,
    ctx: String,
    amount: f64,
    label_id: i32,
    period: String,
    payment_type_id: i32,
) -> SelectCredit {
    use crate::database::schema::credits;
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

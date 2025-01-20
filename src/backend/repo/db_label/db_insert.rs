use crate::backend::repo::schema::*;
use crate::{backend::entity::entity_label::*, backend::repo::db_connect::connect_database};
use diesel::query_dsl::methods::SelectDsl;
use diesel::{RunQueryDsl, SelectableHelper};
use diesel::dsl::max;

pub fn insert_label(id_label: i32, abb_ctx: String) -> SelectLabels {
    let mut conn = connect_database();
    let new_post = InsertLabels { id_label, abb_ctx };
    diesel::insert_into(labels::table)
        .values(new_post)
        .returning(SelectLabels::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn insert_label_name(label: String) -> SelectLabelsName {
    let mut conn = connect_database();

    let max_ord = labels_name::table
        .select(max(labels_name::ord))
        .first::<Option<i32>>(&mut conn)
        .unwrap_or(Some(0));

    let ord = max_ord.unwrap() + 1; // to max ord fro another table
    let show_able = true;
    let new_post = InsertLabelsName {
        label,
        ord,
        show_able,
    };

    diesel::insert_into(labels_name::table)
        .values(new_post)
        .returning(SelectLabelsName::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

use crate::{
    backend::repo::db_connect::connect_database,
    backend::entity::entity_label::*,
};
use diesel::{RunQueryDsl, SelectableHelper};

pub fn insert_label(id_label: i32, abb_ctx: String) -> SelectLabels {
    use crate::backend::repo::schema::labels;
    let mut conn = connect_database();

    let new_post = InsertLabels { id_label, abb_ctx };

    diesel::insert_into(labels::table)
        .values(new_post)
        .returning(SelectLabels::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn insert_label_name(label: String) -> SelectLabelsName {
    use crate::backend::repo::schema::labels_name;
    let mut conn = connect_database();

    let new_post = InsertLabelsName { label };

    diesel::insert_into(labels_name::table)
        .values(new_post)
        .returning(SelectLabelsName::as_returning())
        .get_result(&mut conn)
        .expect("Error saving new post")
}



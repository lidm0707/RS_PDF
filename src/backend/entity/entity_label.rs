use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::labels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertLabels {
    pub id_label: i32,
    pub abb_ctx: String,
}

#[derive(Queryable, Selectable, Debug , Clone )]
#[diesel(table_name = crate::backend::repo::schema::labels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectLabels {
    pub id: i32,
    pub id_label: i32,
    pub abb_ctx: String,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::labels_name)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertLabelsName {
    pub label: String,
}

#[derive(Queryable, Selectable, Debug , Clone )]
#[diesel(table_name = crate::backend::repo::schema::labels_name)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectLabelsName {
    pub id: i32,
    pub label: String,
}

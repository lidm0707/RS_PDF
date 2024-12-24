use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::labels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertLabels {
    pub label: String,
    pub abb_ctx: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::labels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectLabels {
    pub id: i32,
    pub label: String,
    pub abb_ctx: String,
}

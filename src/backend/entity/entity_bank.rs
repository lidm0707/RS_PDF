use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug , Clone )]
#[diesel(table_name = crate::backend::repo::schema::bank)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectBank {
    pub id: i32,
    pub name: String,

}
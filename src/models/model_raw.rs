use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::raws)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertRaws {
    pub date: String,
    pub ctx: String,
    pub amount: f64,
    pub label: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::raws)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectRaws {
    pub id: i32,
    pub date: String,
    pub ctx: String,
    pub amount: f64,
    pub label: String,
}

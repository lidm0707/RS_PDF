use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug , Clone )]
#[diesel(table_name = crate::backend::repo::schema::revenue_type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectRevenueType {
    pub id: i32,
    pub category: String,

}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::revenue_type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertRevenueType{
    pub category: String,
}



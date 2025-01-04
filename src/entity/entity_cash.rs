use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug , Clone )]
#[diesel(table_name = crate::database::schema::cash)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectCash {
    pub id: i32,
    pub date: String,
    pub period: String,
    pub type_cash: String,
    pub label_id: i32,
    pub amount: f64,

}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::cash)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertCash {
    pub date: String,
    pub period: String,
    pub type_cash: String,
    pub label_id: i32,
    pub amount: f64,
}
// id -> Integer,
// date -> Date,
// period -> Text,
// #[sql_name = "type"]
// type_ -> Text,
// label_id -> Integer,
// amount -> Double,
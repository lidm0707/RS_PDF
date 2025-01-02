use diesel::prelude::*;


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::t1)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct InsertT1 {
    pub date_stard: String,
    pub date_end: String,
    pub time: i32,
    pub label_id: i32,
    pub amount: f64,
    pub total: f64,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::t1)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectT1 {
    pub id: i32,
    pub date_stard: String,
    pub date_end: String,
    pub time: i32,
    pub label_id: i32,
    pub amount: f64,
    pub total: f64,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::database::schema::t1_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertT1Items {
    pub date: String,
    pub period: String,
    pub t1_id: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::t1_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectT1Items {
    pub id: i32,
    pub date: String,
    pub period: String,
    pub t1_id: i32,
}

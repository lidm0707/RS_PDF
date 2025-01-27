use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug , Clone )]
#[diesel(table_name = crate::backend::repo::schema::cash_out)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectCashOut {
    pub id: i32,
    pub date: String,
    pub period: String,
    pub label_id: i32,
    pub note: Option<String>,
    pub amount: f64,

}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::cash_out)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertCashOut {
    pub date: String,
    pub period: String,
    pub label_id: i32,
    pub note: Option<String>,
    pub amount: f64,
}


#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::cash_out)]
pub struct GroupBySumCashOut {
    pub label_id: i32,
    pub period: String,
    pub amount: Option<f64>,
    
}

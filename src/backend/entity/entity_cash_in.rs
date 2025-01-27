use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::backend::repo::schema::cash_in)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectCashIn {
    pub id: i32,
    pub date: String,
    pub period: String,
    pub revenue_id: i32,
    pub note: Option<String>,
    pub amount: f64,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::cash_in)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertCashIn {
    pub date: String,
    pub period: String,
    pub revenue_id: i32,
    pub note: Option<String>,
    pub amount: f64,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::cash_in)]
pub struct GroupBySumCashIn {
    pub period: String,
    pub revenue_id: i32,
    pub amount: Option<f64>,
}

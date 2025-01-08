use diesel::prelude::*;


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::repo::schema::installment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct InsertInstallment {
    pub date_stard: String,
    pub date_end: String,
    pub time: i32,
    pub note: String,
    pub label_id: i32,
    pub amount: f64,
    pub total: f64,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::repo::schema::installment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectInstallment {
    pub id: i32,
    pub date_stard: String,
    pub date_end: String,
    pub time: i32,
    pub note: String,
    pub label_id: i32,
    pub amount: f64,
    pub total: f64,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::repo::schema::installment_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertInstallmentItems {
    pub date: String,
    pub period: String,
    pub bank_id:i32,
    pub amount:f64,
    pub installment_id: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::repo::schema::installment_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectInstallmentItems {
    pub id: i32,
    pub date: String,
    pub period: String,
    pub bank_id:i32,
    pub amount:f64,
    pub installment_id: i32,
}

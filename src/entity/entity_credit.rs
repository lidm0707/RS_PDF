use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::repo::schema::credits)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InsertCredit {
    pub date: String,
    pub ctx: String,
    pub amount: f64,
    pub label_id: i32,
    pub period: String,
    pub payment_type_id: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::repo::schema::credits)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectCredit {
    pub id: i32,
    pub date: String,
    pub ctx: String,
    pub amount: f64,
    pub label_id: i32,
    pub period: String,
    pub payment_type_id: i32,
}




#[derive(Queryable, Debug)]
pub struct GroupBySumCredit {
    pub period: String,
    pub label_id: i32,
    pub amount: Option<f64>,
    
}

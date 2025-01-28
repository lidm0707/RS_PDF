use diesel::prelude::*;


#[derive(Insertable, Debug)]
#[diesel(table_name = crate::backend::repo::schema::planing)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct InsertPlanCredit {
    pub period: String,
    pub label_id: i32,
    pub amount: f64,
}



#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::backend::repo::schema::planing)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectPlanCredit {
    pub id: i32,
    pub period: String,
    pub label_id: i32,
    pub amount: f64,
}

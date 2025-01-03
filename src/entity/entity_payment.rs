use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug , Clone )]
#[diesel(table_name = crate::database::schema::payment_type)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SelectPaymentType {
    pub id: i32,
    pub chanel: String,

}
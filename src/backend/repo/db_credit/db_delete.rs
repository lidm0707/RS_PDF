use diesel::prelude::*;
use crate::backend::repo::schema::credits::dsl::*;


use crate::backend::repo::db_connect::connect_database;

pub fn delete_credit( input_id: i32) -> Result<(), anyhow::Error> {
    let mut conn = connect_database();
    diesel::delete(credits.filter(id.eq(input_id)))
        .execute(&mut conn).expect("Error deleting posts");
    
    Ok(())
}
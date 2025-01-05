use diesel::prelude::*;
use crate::database::schema::credits::dsl::*;


use crate::database::db_connect::connect_database;

pub fn delete_credit( input_id: i32) -> Result<(), anyhow::Error> {
    let mut conn = connect_database();
    diesel::delete(credits.filter(id.eq(input_id)))
        .execute(&mut conn).expect("Error deleting posts");
    
    Ok(())
}
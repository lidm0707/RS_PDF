use diesel::prelude::*;

use crate::backend::repo::db_connect::connect_database;
use crate::backend::repo::schema::labels_name::dsl::{*,id as id_LN};
use crate::backend::repo::schema::labels::dsl::{*,id as id_L};

pub fn delete_label( label_id: i32) -> Result<(), anyhow::Error> {
    let mut conn = connect_database();
    diesel::delete(labels.filter(id_L.eq(label_id)))
        .execute(&mut conn).expect("Error deleting posts");
    
    Ok(())
}


pub fn delete_label_name( label_id: i32) -> Result<(), anyhow::Error> {
    let mut conn = connect_database();
    diesel::delete(labels_name.filter(id_LN.eq(label_id)))
        .execute(&mut conn).expect("Error deleting posts");
    
    Ok(())
}
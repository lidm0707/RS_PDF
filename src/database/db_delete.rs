use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::database::db_connect::connect_database;



pub fn delete_label( label_id: i32) -> Result<(), anyhow::Error> {
    use crate::database::schema::labels::dsl::{labels, id};
    let mut conn = connect_database();
    diesel::delete(labels.filter(id.eq(label_id)))
        .execute(&mut conn).expect("Error deleting posts");
    
    Ok(())
}
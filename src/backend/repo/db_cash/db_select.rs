use self::backend::repo::schema::cash::dsl::*;
use crate::backend::entity::entity_cash::*;
use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;
use diesel::dsl::*;

pub fn select_cash() -> Result<Vec<SelectCash>, anyhow::Error> {
    let mut conn = connect_database();

    let results = cash
        .select(SelectCash::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}



pub fn select_cash_groupby_label(start:&str,end:&str) -> Result<Vec<GroupBySumCash>, anyhow::Error> {
    //payment_type:i32
    let mut conn: SqliteConnection = connect_database();
    //.and(payment_type_id.eq(payment_type))
    cash
        .filter(period.ge(start).and(period.le(end)))
        .group_by((label_id, period ))
        .select((label_id, period, sum(amount)))
        .load::<GroupBySumCash>(&mut conn)
        .map_err(Into::into)
}
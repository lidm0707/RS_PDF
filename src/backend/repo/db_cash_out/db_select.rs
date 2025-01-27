use self::backend::repo::schema::cash_out::dsl::*;
use crate::backend::entity::entity_cash_out::*;
use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;
use diesel::dsl::*;

pub fn select_cash_out() -> Result<Vec<SelectCashOut>, anyhow::Error> {
    let mut conn = connect_database();

    let results = cash_out
        .select(SelectCashOut::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}



pub fn select_cash_out_groupby_label(start:&str,end:&str) -> Result<Vec<GroupBySumCashOut>, anyhow::Error> {
    //payment_type:i32
    let mut conn: SqliteConnection = connect_database();
    //.and(payment_type_id.eq(payment_type))
    cash_out
        .filter(period.ge(start).and(period.le(end)))
        .group_by((label_id, period ))
        .select((label_id, period, sum(amount)))
        .load::<GroupBySumCashOut>(&mut conn)
        .map_err(Into::into)
}
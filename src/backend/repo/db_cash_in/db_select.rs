use self::backend::repo::schema::cash_in::dsl::*;
use crate::backend::entity::entity_cash_in::*;
use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;
use diesel::dsl::*;

pub fn select_cash_in() -> Result<Vec<SelectCashIn>, anyhow::Error> {
    let mut conn = connect_database();

    let results = cash_in
        .select(SelectCashIn::as_select())
        .load(&mut conn)
        .expect("Error loading posts");
    Ok(results)
}



pub fn select_cash_in_groupby_label(start:&str,end:&str) -> Result<Vec<GroupBySumCashIn>, anyhow::Error> {
    //payment_type:i32
    let mut conn: SqliteConnection = connect_database();
    //.and(payment_type_id.eq(payment_type))
    cash_in
        .filter(period.ge(start).and(period.le(end)))
        .group_by((revenue_id, period ))
        .select((period,revenue_id, sum(amount)))
        .load::<GroupBySumCashIn>(&mut conn)
        .map_err(Into::into)
}
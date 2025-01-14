use self::backend::repo::schema::credits::dsl::*;
use crate::backend::entity::entity_credit::*;
use crate::*;
use anyhow;
use diesel::dsl::*;
use diesel::prelude::*;
use backend::repo::db_connect::connect_database;

pub fn select_credit() -> Result<Vec<SelectCredit>, anyhow::Error> {
    let mut conn = connect_database();

    let results = credits
        .select(SelectCredit::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

pub fn select_credit_groupby_label(
    start: &str,
    end: &str,
) -> Result<Vec<GroupBySumCredit>, anyhow::Error> {
    //payment_type:i32
    let mut conn: SqliteConnection = connect_database();
    //.and(payment_type_id.eq(payment_type))
    credits
        .filter(period.ge(start).and(period.le(end)))
        .group_by((period, label_id))
        .select((period, label_id, sum(amount)))
        .load::<GroupBySumCredit>(&mut conn)
        .map_err(Into::into)
}

use self::database::schema::credits::dsl::*;
use crate::entity::entity_credit::*;
use crate::*;
use anyhow;
use database::db_connect::connect_database;
use diesel::dsl::sum;
use diesel::prelude::*;

pub fn select_credit() -> Result<Vec<SelectCredit>, anyhow::Error> {
    let mut conn = connect_database();

    let results = credits
        .select(SelectCredit::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(results)
}

pub fn select_credit_groupby_label(start:&str,end:&str) -> Result<Vec<GroupBySumCredit>, anyhow::Error> {
    let mut conn: SqliteConnection = connect_database();

    credits
        .filter(period.ge(start).and(period.le(end)))
        .group_by((label_id, period))
        .select((label_id, period, sum(amount)))
        .load::<GroupBySumCredit>(&mut conn)
        .map_err(Into::into)
}


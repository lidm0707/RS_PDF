use self::backend::repo::schema::planing_credit::dsl::*;
use crate::backend::entity::entity_plan_credit::*;
use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::prelude::*;

pub fn select_plan_credit_where(
    input_period: &str,
    input_label_id: i32,
) -> Result<Vec<SelectPlanCredit>, anyhow::Error> {
    let mut conn = connect_database();

    let result = planing_credit
        .filter(period.eq(input_period).and(label_id.eq(input_label_id)))
        .select(SelectPlanCredit::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}

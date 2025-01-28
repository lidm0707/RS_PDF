use self::backend::repo::schema::planing::dsl::*;
use crate::backend::entity::entity_plan::*;
use crate::*;
use anyhow;
use backend::repo::db_connect::connect_database;
use diesel::dsl::{max, sum};
use diesel::prelude::*;

pub fn select_plan_where(
    input_period: &str,
    input_label_id: i32,
) -> Result<Vec<SelectPlanCredit>, anyhow::Error> {
    let mut conn = connect_database();

    let result = planing
        .filter(period.eq(input_period).and(label_id.eq(input_label_id)))
        .select(SelectPlanCredit::as_select())
        .load(&mut conn)
        .expect("Error loading posts");

    Ok(result)
}

pub fn select_plan_by_period(input_period: &str) -> Result<(String, f64), anyhow::Error> {
    let mut conn = connect_database();

    // Step 1: Get the maximum ID and period for the input period
    let data = planing
        .filter(period.eq(input_period))
        .group_by((period, label_id))
        .select((max(id), period, label_id))
        .load::<(Option<i32>, String, i32)>(&mut conn)?;

    if data.is_empty() {
        return Err(anyhow::anyhow!("No data found for the given period"));
    }

    // Handle the case where max_id is None
    let mut total_amount = 0.0;
    let mut period_main = String::new();

    for (max_id, period_temp, _label_id) in data {
        // Handle the case where max_id is None
        let max_id = max_id.unwrap_or(0);

        // Calculate the total amount for the filtered rows
        let amount_temp: Option<f64> = planing
            .filter(period.eq(&period_temp).and(id.eq(max_id)))
            .select(sum(amount))
            .first(&mut conn)?;

        // Add to the total amount
        total_amount += amount_temp.unwrap_or(0.0);

        if period_main.is_empty() {
            period_main = period_temp.clone();
        }
    }

    // Return the aggregated result as a tuple
    Ok((period_main, total_amount))
}

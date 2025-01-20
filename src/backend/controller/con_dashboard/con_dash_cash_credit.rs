use std::collections::HashMap;

use crate::backend::service::tranformdata::dashboard_cash_credit::table_period_cash_credit;

pub fn get_dashboard_cash_credit(
    start: &str,
    end: &str,
) -> Vec<(String, HashMap<i32, HashMap<String, f64>>)> {
    table_period_cash_credit(&start, &end)
}

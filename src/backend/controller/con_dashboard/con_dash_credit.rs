use crate::backend::service::tranformdata::dashboard_credit::table_period_credit;

pub fn get_dashboard_credit(
    start: &str,
    end: &str,
) -> Result<Vec<(String, Vec<Option<f64>>)>, anyhow::Error> {
    table_period_credit(&start, &end)
}

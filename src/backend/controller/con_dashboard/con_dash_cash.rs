
use crate::backend::service::tranformdata::dashboard_cash::table_period_cash;

pub fn get_dashboard_cash(
    start: &str,
    end: &str,
) -> Result<Vec<(String, Vec<Option<f64>>)>, anyhow::Error> {
    table_period_cash(&start, &end)
}

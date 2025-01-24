use crate::backend::service::tranformdata::dashboard_net::table_period_net;


pub fn get_dashboard_net(
    start: &str,
    end: &str,
) -> Result<Vec<(String, (f64,f64,f64), f64)>, anyhow::Error> {
    let results = table_period_net(&start, &end);
    Ok(results)
}

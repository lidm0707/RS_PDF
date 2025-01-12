use crate::service::aggr::summary_net_period::data_summary_net;

pub fn get_dashboard_net(
    start: &str,
    end: &str,
) -> Result<Vec<(String, f64, f64)>, anyhow::Error> {
    let results = data_summary_net(&start, &end).unwrap();
    Ok(results)
}

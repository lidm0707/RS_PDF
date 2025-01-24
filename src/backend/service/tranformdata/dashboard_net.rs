use crate::backend::service::{
    aggr::summary_net_period::data_summary_net, date::period_vec::generate_month_range,
};

//    let head_label = head.into_iter().map(|data| data.label).collect::<Vec<String>>();

pub fn table_period_net(start: &str, end: &str) -> Vec<(String, (f64, f64, f64), f64)> {
    let mut df = data_summary_net(start, end).unwrap(); // Vec<(String, f64, f64)>
    let rows: Vec<String> = generate_month_range(start, end).unwrap(); // Vec<String>

    for key in rows {
        // Check if `key` exists in `df`. If not, add it.
        if !df.iter().any(|(s, _, _)| s == &key) {
            df.push((key, (0.00, 0.00, 0.00), 0.00));
        }
    }
    df
}

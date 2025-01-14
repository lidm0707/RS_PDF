use anyhow::Error;
use std::collections::HashMap;
use crate::backend::repo::db_dashboard::db_select::summary_revernue;

pub fn data_summary_net(
    start: &str,
    end: &str,
) -> Result<Vec<(String, f64, f64)>, Error> {
    // Group by using a HashMap
    let mut grouped: HashMap<String, (f64, f64)> = HashMap::new();
    let raw_data = summary_revernue(start, end)?;

    for row in raw_data {
        let period = row.0.clone();
        let channel_cash = row.1.clone();
        let amount = row.2.unwrap_or(0.0);

        // Initialize or accumulate amounts based on the period and channel_cash
        let entry = grouped.entry(period).or_insert((0.0, 0.0));

        match channel_cash.as_str() {
            "IN-COME" => entry.0 += amount,  // Add to IN_COME
            "OUT-COME" => entry.1 += amount, // Add to OUT_COME
            _ => {}, // Ignore other values or handle as needed
        }
    }

    // Convert the grouped results into a Vec
    let mut results: Vec<(String, f64, f64)> = grouped
        .into_iter()
        .map(|(period, (in_come, out_come))| (period, in_come, out_come))
        .collect();

    // Sort by period (ascending order)
    results.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(results)
}

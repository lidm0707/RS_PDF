use crate::backend::repo::db_dashboard::db_select::summary_revernue;
use anyhow::Error;
use std::collections::HashMap;

pub fn data_summary_net(start: &str, end: &str) -> Result<Vec<(String, (f64,f64,f64), f64)>, Error> {
    // Group by using a HashMap
    let mut grouped: HashMap<String, ((f64, f64, f64), f64)> = HashMap::new();
    let raw_data = summary_revernue(start, end)?;

    for row in raw_data {
        let period = row.0.clone();
        let channel_cash = row.1.clone();
        let label_id = row.2;
        let amount = row.3.unwrap_or(0.0);

        // Initialize or accumulate amounts based on the period and channel_cash
        let entry = grouped.entry(period).or_insert(((0.0, 0.0, 0.0), 0.0));

        // ('SALARY'),
        // ('EXTRA'),
        // ('REFUND');
        match channel_cash.as_str() {
            "IN-COME" => match label_id {
                1i32 => entry.0 .0 += amount,
                2i32 => entry.0 .1 += amount,
                3i32 => entry.0 .2 += amount,
                _ => {} // Handle all other label_id values
            }, 
            "OUT-COME" => entry.1 += amount, 
            _ => {}                          
        }
    }

    // Convert the grouped results into a Vec
    let mut results: Vec<(String, (f64,f64,f64), f64)> = grouped
        .into_iter()
        .map(|(period, (in_come, out_come))| (period, in_come, out_come))
        .collect();

    // Sort by period (ascending order)
    results.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(results)
}
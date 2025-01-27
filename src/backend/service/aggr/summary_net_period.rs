use crate::backend::repo::{db_cash_in::db_select::select_cash_in_groupby_label, db_dashboard::db_select::summary_revernue};
use anyhow::Error;
use std::collections::HashMap;

pub fn data_summary_net(start: &str, end: &str) -> Result<Vec<(String, (f64,f64,f64), f64)>, Error> {
    // Group by using a HashMap
    let mut grouped: HashMap<String, ((f64, f64, f64), f64)> = HashMap::new();
    let raw_out = summary_revernue(start, end)?;
    let raw_in = select_cash_in_groupby_label(start, end)?;

    for row in raw_out {
        let period = row.0.clone();
        let amount = row.2.unwrap_or(0.0);

        // Initialize or accumulate amounts based on the period and channel_cash
        let entry = grouped.entry(period).or_insert(((0.0, 0.0, 0.0), 0.0));
        entry.1 += amount

    }


    for row in raw_in {
        let period = row.period.clone();
        let revenue_id = row.revenue_id;
        let amount = row.amount.unwrap_or(0.0);
        

        let entry = grouped.entry(period).or_insert(((0.0, 0.0, 0.0), 0.0));
        match revenue_id {
            1 => entry.0 .0 += amount, // 'SALARY'
            2 => entry.0 .1 += amount, // 'EXTRA'
            3 => entry.0 .2 += amount, // 'REFUND'
            _ => {} // Handle all other label_id values
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
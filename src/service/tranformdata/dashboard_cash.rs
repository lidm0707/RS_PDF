use std::collections::HashMap;

use crate::service::{
    aggr::summary_cash::{data_sumary_cash, sort_label_cash},
    date::period_vec::generate_month_range,
};

pub fn table_period_cash(
    start: &str,
    end: &str,
) -> Result<Vec<(String, Vec<Option<f64>>)>, anyhow::Error> {
    // Step 1: Fetch summarized credit data
    let df: HashMap<String, HashMap<i32, f64>> = data_sumary_cash(start, end)?;

    // Step 2: Generate the head (sorted labels)
    let head: Vec<i32> = sort_label_cash(&df)?
        .iter()
        .map(|(label_id, _)| *label_id) // Extract label IDs
        .collect();

    // Step 3: Generate the row (month range)
    let row: Vec<String> = generate_month_range(start, end).unwrap();

    // Step 4: Create a table sorted by head
    let mut table: Vec<(String, Vec<Option<f64>>)> = row
        .iter()
        .map(|month| {
            let row_data = head
                .iter()
                .map(|label_id| {
                    df.get(month) // Access the inner HashMap for the given month
                        .and_then(|inner_map| inner_map.get(label_id)) // Get the amount for the label_id
                        .copied() // Copy the value out of the reference
                        .map(Some) // Wrap in Some
                        .unwrap_or(None) // Default to None if not found
                })
                .collect::<Vec<Option<f64>>>();
            (month.clone(), row_data)
        })
        .collect();

    // Step 5: Insert the first row (Header row)
    let first_row = (
        "DATE".to_string(),
        head.iter().map(|&label_id| Some(label_id as f64)).collect(),
    );
    table.insert(0, first_row);

    // Step 6: Insert the last row (Total row)
    let last: Vec<f64> = sort_label_cash(&df)?
        .iter()
        .map(|(_, amount)| *amount) // Extract amounts from the sorted data
        .collect();

    let last_row = (
        "TOTAL".to_string(),
        last.iter()
            .map(|&amount| Some(amount))
            .collect::<Vec<Option<f64>>>(),
    );

    table.push(last_row);

    Ok(table)
}

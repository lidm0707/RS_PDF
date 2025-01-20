use std::collections::HashMap;

use crate::backend::{
    repo::db_label::db_select::select_labels_name,
    service::{
        aggr::summary_cash_credit::data_sumary_cash_credit,
        date::period_vec::generate_month_range,
    },
};

pub fn table_period_cash_credit(start: &str, end: &str) -> Vec<(String, HashMap<i32, HashMap<String, f64>>)> {
    // Fetch the month range and label data
    let row: Vec<String> = generate_month_range(start, end).unwrap();
    let head = select_labels_name().unwrap();
    let data = data_sumary_cash_credit(start, end).unwrap();

    // Build the data table by iterating over periods and labels
    let data_table: Vec<(String, HashMap<i32, HashMap<String, f64>>)> = row.iter().map(|period| {
        // Create a HashMap for each period
        let mut period_data_map: HashMap<i32, HashMap<String, f64>> = HashMap::new();

        for data_label in &head {
            let mut label_data_map = HashMap::new();

            // Access the period's data and label data
            if let Some(period_data) = data.get(period) {
                if let Some(label_data) = period_data.get(&data_label.id) {
                    // Use default values of 0.0 if the data is missing
                    let cash_amount = label_data.get("cash").unwrap_or(&0.0);
                    let credit_amount = label_data.get("credit").unwrap_or(&0.0);
                    label_data_map.insert("cash".to_string(), *cash_amount);
                    label_data_map.insert("credit".to_string(), *credit_amount);
                }
            }

            // If no data is available, insert default 0.0 values
            if label_data_map.is_empty() {
                label_data_map.insert("cash".to_string(), 0.0);
                label_data_map.insert("credit".to_string(), 0.0);
            }

            // Insert the labelId and corresponding data into the period map
            period_data_map.insert(data_label.id, label_data_map);
        }

        // Return the period and the corresponding HashMap
        (period.clone(), period_data_map)
    }).collect();

    data_table // Return the final result
}

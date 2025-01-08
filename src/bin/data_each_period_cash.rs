use std::collections::HashMap;

use rust_pdf::repo::{
    db_cash::db_select::select_groupby_label, db_credit::db_select::select_credit_groupby_label,
};

fn main() {
    let (start, end) = ("2025-02", "2025-03");
    let data_cash = select_groupby_label(start, end).unwrap_or_else(|err| {
        eprintln!("Error loading cash data: {:?}", err);
        vec![]
    });
    let data_credit = select_credit_groupby_label(start, end).unwrap_or_else(|err| {
        eprintln!("Error loading credit data: {:?}", err);
        vec![]
    });

    // Create a nested HashMap: period -> { label_id -> amount }
    let mut period_map: HashMap<String, HashMap<i32, f64>> = HashMap::new();

    // Process cash data
    for entry in data_cash {
        let period_entry = period_map.entry(entry.period.clone()).or_insert_with(HashMap::new);
        period_entry
            .entry(entry.label_id)
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    // Process credit data
    for entry in data_credit {

        /// add in future use installment plan and avg
        let period_entry = period_map.entry(entry.period.clone()).or_insert_with(HashMap::new);
        period_entry
            .entry(entry.label_id)
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    // Print the nested HashMap
    for (period, labels) in &period_map {
        println!("Period: {}", period);
        for (label_id, amount) in labels {
            println!("  Label ID: {}, Amount: {}", label_id, amount);
        }
    }
}

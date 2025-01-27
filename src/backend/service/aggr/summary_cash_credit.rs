use std::collections::HashMap;

use crate::backend::repo::{
    db_cash_out::db_select::select_cash_out_groupby_label, db_dashboard::db_select::union_installment_credit,
};

pub fn data_sumary_cash_credit(
    start: &str,
    end: &str,
) -> Result<HashMap<String, HashMap<i32, HashMap<String, f64>>>, anyhow::Error> {
    // Fetch cash and credit data
    let data_cash = select_cash_out_groupby_label(start, end)
        .map_err(|err| anyhow::anyhow!("Error loading cash data: {:?}", err))?;
    let data_credit = union_installment_credit(start, end)
        .map_err(|err| anyhow::anyhow!("Error loading credit data: {:?}", err))?;

    // Create a nested HashMap: period -> { label_id -> amount }
    let mut period_map: HashMap<String, HashMap<i32, HashMap<String, f64>>> = HashMap::new();
    // Process cash data
    for entry in data_cash {
        let period_entry = period_map
            .entry(entry.period.clone())
            .or_insert_with(HashMap::new);

        // Insert or modify amount based on label_id
        period_entry
            .entry(entry.label_id)
            .or_insert_with(HashMap::new)
            .entry("cash".to_string()) // Category: "cash"
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    // Process credit data
    for entry in data_credit {
        let period_entry = period_map
            .entry(entry.period.clone())
            .or_insert_with(HashMap::new);

        // Insert or modify amount based on label_id
        period_entry
            .entry(entry.label_id) // Category: "credit"
            .or_insert_with(HashMap::new)
            .entry("credit".to_string())
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    Ok(period_map)
}

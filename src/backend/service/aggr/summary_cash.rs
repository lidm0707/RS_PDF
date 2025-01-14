use std::collections::HashMap;

use crate::backend::repo::{
    db_cash::db_select::select_cash_groupby_label,
    db_credit::db_select::select_credit_groupby_label,
};

pub fn data_sumary_cash(
    start: &str,
    end: &str,
) -> Result<HashMap<String, HashMap<i32, f64>>, anyhow::Error> {
    // Fetch cash and credit data
    let data_cash = select_cash_groupby_label(start, end)
        .map_err(|err| anyhow::anyhow!("Error loading cash data: {:?}", err))?;
    let data_credit = select_credit_groupby_label(start, end)
        .map_err(|err| anyhow::anyhow!("Error loading credit data: {:?}", err))?;

    // Create a nested HashMap: period -> { label_id -> amount }
    let mut period_map: HashMap<String, HashMap<i32, f64>> = HashMap::new();

    // Process cash data
    for entry in data_cash {
        let period_entry = period_map
            .entry(entry.period.clone())
            .or_insert_with(HashMap::new);
        period_entry
            .entry(entry.label_id)
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    for entry in data_credit {
        let label_id = 0;
        let period_entry = period_map
            .entry(entry.period.clone())
            .or_insert_with(HashMap::new);
        period_entry
            .entry(label_id)
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    Ok(period_map)
}

pub fn sort_label_cash(
    data: &HashMap<String, HashMap<i32, f64>>,
) -> Result<Vec<(i32, f64)>, anyhow::Error> {
    let mut aggregated: HashMap<i32, f64> = HashMap::new();

    // Aggregate amounts for each label_id
    for (_, inner_map) in data {
        for (&label_id, &amount) in inner_map {
            *aggregated.entry(label_id).or_insert(0.0) += amount;
        }
    }

    // Convert aggregated HashMap into a Vec and sort by amount (descending)
    let mut sorted: Vec<(i32, f64)> = aggregated.into_iter().collect();

    // Custom sorting: if label_id is 0, prioritize it at the top
    sorted.sort_by(|a, b| {
        if a.0 == 0 {
            std::cmp::Ordering::Less // Move label_id 0 to the top
        } else if b.0 == 0 {
            std::cmp::Ordering::Greater // Keep other values below
        } else {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal) // Sort by amount descending
        }
    });

    Ok(sorted)
}

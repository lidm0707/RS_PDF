use std::{cmp::Ordering, collections::HashMap};

use rust_pdf::{database::db_credit::db_select::select_credit_groupby_label, entity::entity_credit::GroupBySumCredit};




pub fn format_and_sort_period_label_data(
    data: Vec<GroupBySumCredit>,
) -> HashMap<String, Vec<(u32, f64)>> {
    let mut period_data: HashMap<String, HashMap<u32, f64>> = HashMap::new();

    // Aggregate the data by period and label
    for entry in data {
        if let Some(amount) = entry.amount {
            period_data
                .entry(entry.period.clone())
                .or_insert_with(HashMap::new)
                .insert(entry.label_id.try_into().unwrap(), amount);
        }
    }

    // Sort the data within each period
    let mut sorted_period_data: HashMap<String, Vec<(u32, f64)>> = HashMap::new();
    for (period, labels) in period_data {
        let mut sorted_labels: Vec<(u32, f64)> = labels.into_iter().collect();
        sorted_labels.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sorted_period_data.insert(period, sorted_labels);
    }

    sorted_period_data
}


    fn main(){
        println!("{:?}",format_and_sort_period_label_data(select_credit_groupby_label("2025-02","2025-03").unwrap()));
    }
    

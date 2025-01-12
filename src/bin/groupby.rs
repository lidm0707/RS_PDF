// use std::{cmp::Ordering, collections::HashMap};

// use rust_pdf::{database::db_credit::db_select::select_credit_groupby_label, entity::entity_credit::GroupBySumCredit};

// pub fn format_and_sort_period_label_data(
//     data: Vec<GroupBySumCredit>,
// ) -> HashMap<String, Vec<(u32, f64)>> {
//     let mut period_data: HashMap<String, HashMap<u32, f64>> = HashMap::new();

//     // Aggregate the data by period and label
//     for entry in data {
//         if let Some(amount) = entry.amount {
//             period_data
//                 .entry(entry.period.clone())
//                 .or_insert_with(HashMap::new)
//                 .insert(entry.label_id.try_into().unwrap(), amount);
//         }
//     }

//     // Sort the data within each period
//     let mut sorted_period_data: HashMap<String, Vec<(u32, f64)>> = HashMap::new();
//     for (period, labels) in period_data {
//         let mut sorted_labels: Vec<(u32, f64)> = labels.into_iter().collect();
//         sorted_labels.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
//         sorted_period_data.insert(period, sorted_labels);
//     }

//     sorted_period_data
// }

use rust_pdf::{controller::con_dashboard::{con_dash_credit::get_dashboard_credit, con_dash_net::get_dashboard_net}, repo::db_dashboard::db_select::{summary_revernue, union_installment_credit}, service::aggr::summary_net_period::data_summary_net};

//println!("{:?}",format_and_sort_period_label_data(select_credit_groupby_label("2025-02","2025-03").unwrap()));
fn main() {
    let start = "2025-01";
    let end = "2025-05";
    // println!("{:?}",union_installment_credit(start,end));
    // println!("{:?}",get_dashboard_net(start,end));
    println!("{:?}",get_dashboard_net(start, end));
    println!("{:?}",summary_revernue(start, end));
}
use crate::{
    repo::db_credit::db_select::select_credit_groupby_label,
    entity::entity_credit::GroupBySumCredit,
};
use anyhow;
use std::{cmp::Ordering, collections::HashMap};

pub fn data_sumary_credit(start: &str, end: &str) -> Result<Vec<GroupBySumCredit>, anyhow::Error> {
    let mut data: Vec<GroupBySumCredit> = select_credit_groupby_label(start, end).unwrap();
    // payment 1 = pay all 2 = installment check
    // I think if amount is current should show payment 1 and 2
    // in the futer show form installment
    data.sort_by(|a, b| {
        let period_cmp = b.period.cmp(&a.period);
        if period_cmp == Ordering::Equal {
            b.amount
                .unwrap_or(0.0) // ใช้ค่าเริ่มต้นหากเป็น None
                .partial_cmp(&a.amount.unwrap_or(0.0)) // ใช้ partial_cmp เพื่อเปรียบเทียบค่าแบบ f64
                .unwrap_or(Ordering::Equal) // หากค่าไม่สามารถเปรียบเทียบได้ ให้ถือว่าเท่ากัน
        } else {
            period_cmp
        }
    });
    Ok(data)
}

pub fn sort_label_credit(data: &Vec<GroupBySumCredit>) -> Result<Vec<(i32, f64)>,anyhow::Error> {

    let mut aggregated: HashMap<i32, f64> = HashMap::new();

    for entry in data {
        if let Some(amount) = entry.amount {
            *aggregated
                .entry(entry.label_id.try_into().unwrap())
                .or_insert(0.0) += amount;
        }
    }
    let mut sorted: Vec<(i32, f64)> = aggregated.into_iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    Ok(sorted)
}



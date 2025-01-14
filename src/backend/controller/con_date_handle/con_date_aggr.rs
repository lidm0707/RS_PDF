use crate::backend::service::date::{add::month_add, diff::diff_month};

pub fn set_month_add(date: &str, add: &str) -> String{
    month_add(date, add)
}


pub fn set_date_diff(date1: &str, date2: &str) -> Result<i32,anyhow::Error>{
    let result = diff_month(date1, date2).unwrap();
    Ok(result)
}
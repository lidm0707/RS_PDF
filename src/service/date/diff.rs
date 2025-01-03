use anyhow;
use chrono::prelude::*;

pub fn diff_month(date1: String, date2: String) -> Result<i32, anyhow::Error> {
    let date1 = NaiveDate::parse_from_str(&date1, "%Y-%m-%d")?;
    let date2 = NaiveDate::parse_from_str(&date2, "%Y-%m-%d")?;
    let month1: i32 = date1.month() as i32;
    let month2: i32 = date2.month() as i32;

    Ok((month2 - month1).abs() +1)
}

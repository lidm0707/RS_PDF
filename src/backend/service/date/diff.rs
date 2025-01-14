use anyhow;
use chrono::prelude::*;

pub fn diff_month(date1: &str, date2: &str) -> Result<i32, anyhow::Error> {
    let date1 = NaiveDate::parse_from_str(&date1, "%Y-%m-%d")?;
    let date2 = NaiveDate::parse_from_str(&date2, "%Y-%m-%d")?;
    let year1: i32 = date1.year();
    let year2: i32 = date2.year();
    let month1: i32 = date1.month() as i32;
    let month2: i32 = date2.month() as i32;

    Ok((year2 - year1) * 12 + (month2 - month1) +1)
}

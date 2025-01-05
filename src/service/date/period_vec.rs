use chrono::{NaiveDate, Datelike};

pub fn generate_month_range(start: &str, end: &str) -> Result<Vec<String>, &'static str> {
    let start_date = NaiveDate::parse_from_str(&format!("{}-01", start), "%Y-%m-%d")
        .map_err(|_| "Invalid start date format")?;
    let end_date = NaiveDate::parse_from_str(&format!("{}-01", end), "%Y-%m-%d")
        .map_err(|_| "Invalid end date format")?;
    
    if start_date > end_date {
        return Err("Start date must be earlier than or equal to the end date");
    }

    let mut current_date = start_date;
    let mut result = Vec::new();

    while current_date <= end_date {
        result.push(format!("{}-{:02}", current_date.year(), current_date.month()));
        current_date = current_date
            .with_month(current_date.month() + 1)
            .unwrap_or_else(|| current_date.with_month(1).unwrap().with_year(current_date.year() + 1).unwrap());
    }

    Ok(result)
}

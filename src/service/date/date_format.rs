use chrono::prelude::*;
pub fn format_date(date: &str) -> String {
    let date = NaiveDate::parse_from_str(date, "%d/%m/%y").unwrap();
    date.format("%Y-%m-%d").to_string()
}

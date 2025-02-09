use chrono::prelude::*;
pub fn format_date(date: &str) -> String {
    match NaiveDate::parse_from_str(date, "%d/%m/%y") {
        Ok(data) => data.format("%Y-%m-%d").to_string(),
        Err(err) => {
            println!("Error: {} {:?}", err, date);
            "".to_string()
        }
    }
}

pub fn format_period(date: &str) -> String {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(d) => d.format("%Y-%m").to_string(),
        Err(err) => {
            println!("Error: {} {:?}", err, date);
            "".to_string()
        }
    }
}

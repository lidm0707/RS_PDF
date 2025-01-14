use chrono::prelude::*;

pub fn month_add(date: &str, add: &str) -> String {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(data) => {
            let add_months = match add.parse::<i32>() {
                Ok(num) => num,
                Err(err) => {
                    println!("Error: {} {:?}", err, add);
                    return "".to_string();
                }
            };

            let total_months = data.year() * 12 + (data.month() as i32 - 1) + add_months;
            let new_year = total_months / 12;
            let new_month = (total_months % 12) + 1;

            match NaiveDate::from_ymd_opt(new_year, new_month as u32, data.day()) {
                Some(new_date) => new_date.format("%Y-%m-%d").to_string(),
                None => {
                    println!("Invalid date after adding months");
                    "".to_string()
                }
            }
        },
        Err(err) => {
            println!("Error: {} {:?}", err, date);
            "".to_string()
        }
    }
}
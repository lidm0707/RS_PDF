use chrono::prelude::*;

fn main() {
    // let utc: DateTime<Utc> = Utc::now();
    // let utc_plus_7 = FixedOffset::east_opt(7 * 3600).unwrap();
    // let now_in_utc_plus_7 = utc.with_timezone(&utc_plus_7);
    // println!("{}",now_in_utc_plus_7);
    // println!("{}-{}",now_in_utc_plus_7.year(),now_in_utc_plus_7.month());

    // let date = "24/12/24";
    // let date = NaiveDate::parse_from_str(date, "%d/%m/%y").unwrap();
    // let str_date = date.format("%Y-%m-%d").to_string();
    // println!("{}",str_date);

    let date1 = "2024-12-24";
    let date1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d").unwrap();

    let date2 = "2025-01-01";
    let date2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d").unwrap();
    println!("{}",date1.month());
    println!("{}",date2.month());
    let month1:i32 = date1.month() as i32; 
    let month2:i32 = date2.month() as i32 ;

    let diff_month: i32 = (month2 - month1).abs();

    println!("{}",diff_month);
}

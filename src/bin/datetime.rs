use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    let utc_plus_7 = FixedOffset::east_opt(7 * 3600).unwrap();
    let now_in_utc_plus_7 = utc.with_timezone(&utc_plus_7);
    println!("{}",now_in_utc_plus_7);
    println!("{}-{}",now_in_utc_plus_7.year(),now_in_utc_plus_7.month());
}
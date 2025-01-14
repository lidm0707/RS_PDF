
use chrono::prelude::*;

pub fn thai_now() -> DateTime<FixedOffset> {
    let now: DateTime<Utc> = Utc::now();
    let now_puls7 = FixedOffset::east_opt(7 * 3600).unwrap();
    now.with_timezone(&now_puls7)
}


pub fn thai_now_string() -> String {
    let now: DateTime<Utc> = Utc::now();
    let now_plus7 = FixedOffset::east_opt(7 * 3600).unwrap();
    let local_time = now.with_timezone(&now_plus7);
    local_time.format("%y-%m-%d").to_string()
}
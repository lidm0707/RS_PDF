use chrono::{DateTime, FixedOffset};

use crate::backend::service::date::now::{thai_now, thai_now_string};

pub fn get_thai_now() -> DateTime<FixedOffset> {
    thai_now()
}

pub fn get_thai_now_string() -> String {
    thai_now_string()
}

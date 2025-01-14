use crate::backend::service::date::date_format::{format_date, format_period};



pub fn get_format_date(data:&str) -> String {
    format_date(data)
}

pub fn get_format_period(data:&str) -> String {
    format_period(data)
}
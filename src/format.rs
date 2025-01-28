
use num_format::{Locale, ToFormattedString};

pub fn format_with_separator(num: &f64) -> String {
        let whole_part = (num.trunc() as u64).to_formatted_string(&Locale::th);
        let fractional_part = (num.fract() * 100.0).round() as u64;
        format!("{}.{:02}", whole_part, fractional_part)
    
}
use num_format::{Locale, ToFormattedString};

pub fn format_with_separator(num: &f64) -> String {
    let is_negative = *num < 0.0;
    let abs_num = num.abs(); // ใช้ค่าสัมบูรณ์ของตัวเลข
    let whole_part = (abs_num.trunc() as u64).to_formatted_string(&Locale::th);
    let fractional_part = (abs_num.fract() * 100.0).round() as u64;
    
    if is_negative {
        format!("-{}.{}", whole_part, fractional_part)
    } else {
        format!("{}.{}", whole_part, fractional_part)
    }
}

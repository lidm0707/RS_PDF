use anyhow;
use regex::Regex;

use crate::repo::db_label::db_select::select_labels;


pub fn search_labels(ctx: &str) -> Result<Option<(i32,i32)>, anyhow::Error> {
    let label_items = select_labels()?;
    let ctx_lower = ctx.to_lowercase();

    // Regex สำหรับตรวจสอบ channel
    let re_channel = Regex::new(r"(?i)\d{2}/10|INSTALLMENT")?;

    for item in label_items {
        let channel = if re_channel.is_match(&ctx_lower) {
            2 // Installment
        } else {
            1 // Credit
        };

        let regex_pattern = format!(r"(?i).*{}.*", regex::escape(&item.abb_ctx));
        if Regex::new(&regex_pattern)?.is_match(&ctx_lower) {
            return Ok(Some((item.id_label, channel)));
        }
    }

    // ไม่พบการจับคู่
    Ok(Some((0, 0)))
}

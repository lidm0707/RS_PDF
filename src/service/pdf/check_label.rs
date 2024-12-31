use anyhow;

use crate::database::db_select::select_labels;

pub fn search_labels(ctx: &str) -> Result<Option<i32>, anyhow::Error> {
    let label_items = select_labels().unwrap();
    if let Some(macth) = label_items
        .iter()
        .find(|item| ctx.to_lowercase().contains(&item.abb_ctx.to_lowercase()))
    {
        Ok(Some(macth.id_label))
    } else {
        Ok(Some(0i32))
    }
}

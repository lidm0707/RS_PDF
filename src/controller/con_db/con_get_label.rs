use crate::{entity::entity_label::SelectLabelsName, repo::db_label::db_select::select_labels_name_where};

pub fn get_label_where(input_id:i32) -> Result<Vec<SelectLabelsName>, anyhow::Error> {
    select_labels_name_where(input_id)
}
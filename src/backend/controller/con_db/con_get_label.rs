use crate::backend::{
    model::model_label::{ModelLabels, ModelLabelsName},
    service::sv_data::sv_get_label::{
        sv_get_count_labels_where, sv_get_label_name, sv_get_label_name_where, sv_get_labels, sv_get_labels_where
    },
};

pub fn get_label_name_where(input_id: i32) -> Result<Vec<ModelLabelsName>, anyhow::Error> {
    let result = sv_get_label_name_where(input_id).unwrap();

    Ok(result)
}

pub fn get_label_name() -> Result<Vec<ModelLabelsName>, anyhow::Error> {
    let result = sv_get_label_name().unwrap();

    Ok(result)
}

pub fn get_labels_where(input_id: i32) -> Result<Vec<ModelLabels>, anyhow::Error> {
    let result = sv_get_labels_where(input_id).unwrap();

    Ok(result)
}

pub fn get_count_labels_where(input_id: i32) -> Result<i64, anyhow::Error> {
    let result = sv_get_count_labels_where(input_id).unwrap();
    Ok(result)
}

pub fn get_labels() -> Result<Vec<ModelLabels>, anyhow::Error> {
    let result = sv_get_labels().unwrap();

    Ok(result)
}

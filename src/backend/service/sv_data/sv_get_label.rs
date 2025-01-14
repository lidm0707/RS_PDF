use crate::backend::{
    model::model_label::{ModelLabels, ModelLabelsName},
    repo::db_label::db_select::{
        count_labels_where, select_labels, select_labels_name, select_labels_name_where, select_labels_where,
    },
};

pub fn sv_get_label_name_where(input_id: i32) -> Result<Vec<ModelLabelsName>, anyhow::Error> {
    let raw_data = select_labels_name_where(input_id).unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelLabelsName {
            id: raw.id,
            label: raw.label,
        })
        .collect();

    Ok(result)
}

pub fn sv_get_label_name() -> Result<Vec<ModelLabelsName>, anyhow::Error> {
    let raw_data = select_labels_name().unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelLabelsName {
            id: raw.id,
            label: raw.label,
        })
        .collect();

    Ok(result)
}


pub fn sv_get_labels_where(input_id:i32) -> Result<Vec<ModelLabels>, anyhow::Error> {
    let raw_data = select_labels_where(input_id).unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelLabels {
             id: raw.id,
             id_label: raw.id_label,
             abb_ctx: raw.abb_ctx,
        })
        .collect();

    Ok(result)
}

pub fn sv_get_count_labels_where(input_id: i32) -> Result<i64, anyhow::Error> {
    let result = count_labels_where(input_id).unwrap();
    Ok(result)
}

pub fn sv_get_labels() -> Result<Vec<ModelLabels>, anyhow::Error> {
    let raw_data = select_labels().unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelLabels {
            id: raw.id,
            id_label: raw.id_label,
            abb_ctx: raw.abb_ctx,
        })
        .collect();
    Ok(result)
}

// //    pub id: i32,
// pub id_label: i32,
// pub abb_ctx: String,
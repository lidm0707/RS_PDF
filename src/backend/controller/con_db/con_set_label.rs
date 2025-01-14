use crate::backend::{
    model::model_label::{ModelLabels, ModelLabelsName},
    service::sv_data::sv_set_label::{sv_remove_label, sv_set_label, sv_set_label_name},
};

pub fn set_label(id_label: i32, abb_ctx: String) -> ModelLabels {
    let raw_data = sv_set_label(id_label, abb_ctx);
    let result = ModelLabels {
        id: raw_data.id,
        id_label: raw_data.id_label,
        abb_ctx: raw_data.abb_ctx,
    };

    result
}

pub fn set_label_name(label: String) -> ModelLabelsName {
    let raw_data = sv_set_label_name(label);
    let result = ModelLabelsName {
        id: raw_data.id,
        label: raw_data.label,
    };

    result
}

pub fn remove_label(label_id: i32) {
    let _ = sv_remove_label(label_id);
}

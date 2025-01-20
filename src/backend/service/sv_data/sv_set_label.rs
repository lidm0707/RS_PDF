use crate::backend::{
    model::model_label::{ModelLabels, ModelLabelsName},
    repo::db_label::{
        db_delete::delete_label,
        db_insert::{insert_label, insert_label_name},
    },
};

pub fn sv_set_label(id_label: i32, abb_ctx: String) -> ModelLabels {
    let raw_data = insert_label(id_label, abb_ctx);
    let result = ModelLabels {
        id: raw_data.id,
        id_label: raw_data.id_label,
        abb_ctx: raw_data.abb_ctx,
    };

    result
}

pub fn sv_set_label_name(label: String) -> ModelLabelsName {
    let raw_data =  insert_label_name(label);
    let result = ModelLabelsName {
        id: raw_data.id,
        label: raw_data.label,
        ord:raw_data.ord,
        show_able:raw_data.show_able,
    };

    result
}

pub fn sv_remove_label(label_id: i32) {
    let _ = delete_label(label_id).unwrap();
}

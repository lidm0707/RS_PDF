use dioxus::prelude::*;
use crate::{component::upload::FileUpload, database::db_select::select_labels_name_where};

#[component]
pub fn UploadTable(file_upload: FileUpload) -> Element {
    rsx! {
        div { class: "table-container",
            table {
                thead {
                    tr {
                        th { {"DATE"} }
                        th { {"CTX"} }
                        th { {"AMOUNT"} }
                        th { {"LABEL_ID"} }
                        th { {"PERIOD"} }
                    }
                }
                tbody {
                    {
                        file_upload
                            .data
                            .read()
                            .clone()
                            .iter()
                            .map(|raw| {
                                let r2 = format!("{:.2}", raw.amount);
                                let l_id = raw.label_id as i32;
                                let label_name = select_labels_name_where(l_id).unwrap()[0].label.clone();
                                rsx! {
                                    tr {
                                        td { "{raw.date}" }
                                        td { "{raw.ctx}" }
                                        td { class: "text-right", "{r2}" }
                                        td { "{label_name}" }
                                        td { "{raw.period}" }
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}
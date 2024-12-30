
use diesel::SqliteConnection;
use dioxus::prelude::*;
use crate::{database::{db_connect::connect_database, db_delete::delete_label, db_select::select_labels}, models::{model_label::SelectLabels, model_raw::SelectRaws}};




#[component]
pub fn LabelTable(data_table: Signal<Vec<SelectLabels>>) -> Element {
    let mut updated_data = use_signal(|| select_labels().expect("Failed to load labels"));
    rsx! {
        div { class: "table-container h-1/6",
            table {
                thead {
                    tr {
                        th { {"LABEL"} }
                        th { {"CTX"} }
                        th { class: "w-1/6", {"setting"} }
                    }
                }
                tbody {
                    {
                        data_table
                            .iter()
                            .map(|label| {
                                let id = label.id.clone();  

                                rsx! {
                                    tr { id: "{label.id}",
                                        td { "{label.label}" }
                                        td { "{label.abb_ctx}" }
                                        td { class: "w-1/6 ",
                                            button {
                                                class: "btnEdit",
                                                onclick: move |_| {
                                                     let _ = delete_label(id.clone());
                                                     let _ = &mut data_table.set(updated_data.write().to_vec());
                                                },
                                                {"DELETE"}
                                            }
                                        }
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}

#[component]
pub fn RawTable(data_table: Signal<Vec<SelectRaws>>) -> Element {
    rsx! {
        div { class: "table-container",
            table {
                thead {
                    tr {
                        th { {"DATE"} }
                        th { {"CTX"} }
                        th { {"AMOUNT"} }
                        th { {"LABEL"} }
                    }
                }
                tbody {
                    {
                        &mut data_table
                            .iter()
                            .map(|raw| {
                                let r2 = format!("{:.2}", raw.amount);
                                rsx! {
                                    tr {
                                        td { "{raw.date}" }
                                        td { "{raw.ctx}" }
                                        td { class: "text-right", "{r2}" }
                                        td { "{raw.label}" }
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}

#[component]
pub fn UploadTable(files_uploaded: Vec<(String, String, f64)>) -> Element {
    rsx! {
        div { class: "table-container",
            table {
                thead {
                    tr {
                        th { {"DATE"} }
                        th { {"CTX"} }
                        th { {"AMOUNT"} }
                    }
                }
                tbody {
                    {
                        &mut files_uploaded
                            .iter()
                            .map(|raw| {
                                let r2 = format!("{:.2}", raw.2);
                                rsx! {
                                    tr {
                                        td { "{raw.0}" }
                                        td { "{raw.1}" }
                                        td { class: "text-right", "{r2}" }
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}

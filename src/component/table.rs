
use dioxus::prelude::*;
use crate::{database::{db_delete::delete_label, db_select::select_labels}, entity::{entity_label::SelectLabels, entity_credit::SelectCredit}};

#[component]
pub fn LabelTable(data_table: Signal<Vec<SelectLabels>>) -> Element {
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
                                                     let _ = &mut data_table.set(select_labels().expect("Failed to load labels"));
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
pub fn RawTable(data_table: Signal<Vec<SelectCredit>>) -> Element {
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

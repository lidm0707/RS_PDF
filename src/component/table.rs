use dioxus::prelude::*;

use crate::models::{model_label::SelectLabels, model_raw::SelectRaws};

#[component]
pub fn LabelTable(data_table: Signal<Vec<SelectLabels>>) -> Element {
    rsx! {

        div { class:"table-container",
            table {
                thead {
                    tr {
                        th { {"LABEL"} }
                        th { {"CTX"} }
                    }
                }
                tbody {
                    {
                        &mut data_table
                            .iter()
                            .map(|label| {
                                rsx! {
                                    tr {
                                        td { "{label.label}" }
                                        td { "{label.abb_ctx}" }
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
        div {class:"table-container",
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
                                let r2 = format!("{:.2}",raw.amount);
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
pub fn UploadTable(files_uploaded:Vec<(String, String, f64)>)-> Element{
    rsx!{
        div { class: "table-container h-3/6",
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
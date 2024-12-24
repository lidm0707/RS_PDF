use dioxus::prelude::*;

use crate::models::{model_label::SelectLabels, model_raw::SelectRaws};

#[component]
pub fn Label(table: Signal<Vec<SelectLabels>>) -> Element {
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
                        &mut table
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
pub fn Raw(table: Signal<Vec<SelectRaws>>) -> Element {
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
                        &mut table
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

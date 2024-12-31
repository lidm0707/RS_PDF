use dioxus::prelude::*;

use crate::{
    database::{
        db_delete::{delete_label, delete_label_name},
        db_select::{count_labels_where, select_labels_name, select_labels_where},
    },
    entity::{
        entity_credit::SelectCredit,
        entity_label::{SelectLabels, SelectLabelsName},
    }
};






/*                    {
    data_table
        .iter()
        .map(|label| {
            let id = label.id.clone();

            rsx! {
                tr { id: "{label.id}",
                    td { "{label.id_label}" }
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
} */

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







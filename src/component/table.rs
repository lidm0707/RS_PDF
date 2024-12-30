use crate::{
    database::{
        db_delete::{delete_label, delete_label_name},
        db_select::{count_labels_where, select_labels_name, select_labels_where},
    },
    entity::{
        entity_credit::SelectCredit,
        entity_label::SelectLabelsName,
    },
};
use dioxus::prelude::*;
//select_labels_name
#[component]
pub fn LabelTable(data_table: Signal<Vec<SelectLabelsName>>) -> Element {
    let mut swicth_show = use_signal(|| true);
    let mut  id_show = use_signal(|| 1);
    rsx! {
        div { class: "table-container h-4/6",
            table {
                if *swicth_show.read() {
                    thead {
                        tr {
                            th { {"LABEL"} }
                            th { {"amount"} }
                            th { class: "w-1/6", {"setting"} }
                        }
                    }
                    tbody {
                        {
                            data_table
                                .iter()
                                .map(|label| {
                                    let id = label.id.clone();
                                    let amount = count_labels_where(id).expect("Failed to load labels");
                                    rsx! {
                                        tr { id: "{label.id}",
                                            td {
                                                onclick: move |_| {
                                                    swicth_show.set(false);
                                                    id_show.set(id);
                                                },
                                                "{label.label}"
                                            }
                                            td { {amount.to_string()} }
                                            td { class: "w-1/6 ",
                                                button {
                                                    class: "btnEdit",
                                                    onclick: move |_| {
                                                        if amount == 0 {
                                                            let _ = delete_label_name(id.clone());
                                                            let _ = &mut data_table
                                                                .set(select_labels_name().expect("Failed to load labels"));
                                                        } else {
                                                            println!("{}", amount);
                                                            println!("can't delete");
                                                         }
                                                    },
                                                    {"DELETE"}
                                                }
                                            }
                                        }
                                    }
                                })
                        }
                    }
                } else {
                    thead {
                        tr {
                            th { {"LABEL"} }
                            th { class: "w-1/6", {"setting"} }
                        }
                    }
                    tbody {
                        {
                            select_labels_where(*id_show.read())
                                .expect("Failed to load labels")
                                .iter()
                                .map(|label| {
                                    let id = label.id.clone();
                                    rsx! {
                                        tr { id: "{label.id}",
                                            td {
                                                onclick: move |_| {
                                                    swicth_show.set(true);
                                                },
                                                "{label.abb_ctx}"
                                            }
                                            td { class: "w-1/6 ",
                                                button {
                                                    class: "btnEdit",
                                                    onclick: move |_| {
                                                        let _ = delete_label(id.clone());
                                                        let _ = &mut data_table
                                                            .set(select_labels_name().expect("Failed to load labels"));
                                                        let id_value = *id_show.read();
                                                        let _ = id_show.set(id_value);
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
    }


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

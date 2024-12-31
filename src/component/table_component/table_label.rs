use dioxus::prelude::*;

use crate::{
    database::{
        db_delete::{delete_label, delete_label_name},
        db_select::{count_labels_where, select_labels_name, select_labels_where},
    },
    entity::entity_label::{SelectLabels, SelectLabelsName},
};

#[component]
pub fn LabelTable(
    updated_data: Signal<Vec<SelectLabelsName>>,
    id_show: Signal<i32>,
    table_ctx: Signal<Vec<SelectLabels>>,
    set_id_show:Callback<i32>,
) -> Element {
    rsx! {
        div { class: "table-container h-4/6",
            table {
                if *id_show.read() == 0 {

                    thead {
                        tr {
                            th { {"LABEL"} }
                            th { {"amount"} }
                            th { class: "w-1/6", {"setting"} }
                        }
                    }
                    tbody {
                        {
                            updated_data
                                .iter()
                                .map(|label| {
                                    let id = label.id.clone();
                                    let amount = count_labels_where(id).expect("Failed to load labels");
                                    rsx! {
                                        tr { id: "{label.id}",
                                            td {
                                                onclick: move |_| {
                                                    set_id_show.call(id);
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
                                                            let _ = &mut updated_data
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
                            let _ = table_ctx
                                .set(select_labels_where(*id_show.read()).expect("Failed to load labels"));
                            table_ctx
                                .iter()
                                .map(|label| {
                                    let id = label.id.clone();
                                    rsx! {
                                        tr { id: "{label.id}",
                                            td {
                                                onclick: move |_| {
                                                    set_id_show.call(0);

                                                },
                                                "{label.abb_ctx}"
                                            }
                                            td { class: "w-1/6 ",
                                                button {
                                                    class: "btnEdit",
                                                    onclick: move |_| {
                                                        let _ = delete_label(id.clone());
                                                        let _ = &mut table_ctx
                                                            .set(select_labels_where(*id_show.read()).expect("Failed to load labels"));
                                                        let id_value = *id_show.read();
                                                        set_id_show.call(id_value);

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

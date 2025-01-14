use dioxus::prelude::*;

use crate::backend::{
    controller::con_db::{
        con_get_label::{get_count_labels_where, get_label_name, get_labels_where},
        con_set_label::remove_label,
    },
    model::model_label::{ModelLabels, ModelLabelsName},
};

#[component]
pub fn LabelTable(
    updated_data: Signal<Vec<ModelLabelsName>>,
    id_show: Signal<i32>,
    table_ctx: Signal<Vec<ModelLabels>>,
    set_id_show: Callback<i32>,
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
                                    let amount = get_count_labels_where(id).expect("Failed to load labels");
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
                                                            let _ = remove_label(id.clone());
                                                            let _ = &mut updated_data
                                                                .set(get_label_name().expect("Failed to load labels"));
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
                                .set(get_labels_where(*id_show.read()).expect("Failed to load labels"));
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
                                                        let _ = remove_label(id.clone());
                                                        let _ = &mut table_ctx
                                                            .set(get_labels_where(*id_show.read()).expect("Failed to load labels"));
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

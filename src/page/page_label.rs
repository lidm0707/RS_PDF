use crate::{
    component::table_component::table_label::LabelTable,
    database::{
        db_insert::{insert_label, insert_label_name},
        db_select::{select_labels_name, select_labels_where},
    },
    entity::entity_label::{SelectLabels, SelectLabelsName},
};
use dioxus::prelude::*;

pub fn content_label() -> Element {
    let mut show_modal: Signal<bool> = use_signal(|| false);
    let mut id_show: Signal<i32> = use_signal(|| 0);
    let mut updated_data: Signal<Vec<SelectLabelsName>> =
        use_signal(|| select_labels_name().expect("Failed to load labels"));
    let mut table_ctx: Signal<Vec<SelectLabels>> =
        use_signal(|| select_labels_where(*id_show.read()).expect("Failed to load labels"));
        //impl FnMut(i32)
    
    let set_id_show: Callback<i32> = use_callback(move |id: i32| {
        id_show.set(id);
    });

    rsx! {
        div {
            id: "modal",
            class: "fixed inset-0  place-items-center place-content-center  bg-black bg-opacity-50",
            visibility: if *show_modal.read() { "visible" } else { "hidden" },
            div { class: "h-fit bg-white rounded-lg shadow-lg max-w-md w-full p-6",
                div {
                    div { class: " flex items-center justify-between mb-4",
                        h2 { class: "text-xl font-semibold", {"modal title"} }
                        button {
                            class: "text-gray-400 hover:text-gray-600",
                            onclick: move |_| {
                                println!("{}", id_show.read());
                                show_modal.set(false);
                            },
                            {"btn"}
                        }
                    }
                }
                if *id_show.read() == 0 {
                    form {
                        class: "",
                        onsubmit: move |evt| {
                            let label_name: String = evt.data.values()["label"].as_value();
                            insert_label_name(label_name);
                            show_modal.set(false);
                            updated_data.set(select_labels_name().expect("Failed to load labels"));
                        },
                        div {
                            div { class: "flex justify-center",
                                input {
                                    class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                    name: "label", // Add name attribute to capture data
                                    r#type: "text",
                                    value: "",
                                }
                            }
                        }
                        div { class: "flex justify-end mr-2",
                            input {
                                class: "",
                                r#type: "submit",
                                class: "btnEdit mt-3 mb-3 mr-10",
                            }
                        }
                    }
                } else {
                    form {
                        class: "",
                        onsubmit: move |evt| {
                            let id_label: i32 = evt
                                .data
                                .values()["id_label"]
                                .as_value()
                                .parse::<i32>()
                                .unwrap_or(0i32);
                            let abb_ctx: String = evt
                                .data
                                .values()["abb_ctx"]
                                .as_value()
                                .parse()
                                .unwrap_or("".to_string());
                            insert_label(id_label, abb_ctx);
                            show_modal.set(false);
                            table_ctx
                                .set(select_labels_where(*id_show.read()).expect("Failed to load labels"));
                        },
                        div {
                            div { class: "flex justify-center",
                                input {
                                    class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                    name: "id_label", // Add name attribute to capture data
                                    r#type: "text",
                                    value: "",
                                }
                            }
                            div { class: "flex justify-center",
                                input {
                                    class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                    name: "abb_ctx", // Add name attribute to capture data
                                    r#type: "text",
                                    value: "",
                                }
                            }
                        }
                        div { class: "flex justify-end mr-2",
                            input {
                                class: "",
                                r#type: "submit",
                                class: "btnEdit mt-3 mb-3 mr-10",
                            }
                        }
                    }
                }
            }
        }
        div { class: "content",

            div { class: "summary" }
            div { class: "control flex justify-end",
                button {
                    class: "btnEdit mt-3 mb-3 mr-10",
                    onclick: move |_| {
                        show_modal.set(true);
                    },
                    {"button"}
                }
            }
            LabelTable { updated_data, id_show, table_ctx ,set_id_show }
        }
    }
}

/*
updated_data: updated_data,
id_show: id_show,
table_ctx: table_ctx,
*/

use dioxus::prelude::*;
use crate::{component::table::LabelTable, database::{db_insert::insert_label, db_select::{select_labels, select_labels_name}}};




pub fn content_label() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut updated_data = use_signal(|| select_labels_name().expect("Failed to load labels"));
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
                                show_modal.set(false);
                            },
                            {"btn"}
                        }
                    }
                }

                form { class: "", onsubmit: move |evt| { 
                    let id_label:i32 = evt.data.values()["id_label"].as_value().parse::<i32>().unwrap_or(0i32);
                    let abb_ctx: String = evt.data.values()["abb_ctx"].as_value().parse().unwrap_or("".to_string());
                    insert_label(id_label, abb_ctx);
                    show_modal.set(false);

                    // Refresh the table data
                    updated_data.set(select_labels_name().expect("Failed to load labels"));
                },
                    div {
                        div { class: "flex justify-center",
                            input {
                                class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                name: "id_label", // Add name attribute to capture data
                                r#type:"text",
                                value: "",
                            }
                        }
                        div { class: "flex justify-center",
                            input {
                                class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                name: "abb_ctx", // Add name attribute to capture data
                                r#type:"text",
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
            LabelTable { data_table: updated_data}
        }
    }
}
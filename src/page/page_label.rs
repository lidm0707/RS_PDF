use crate::{
    backend::controller::con_db::{
        con_get_label::{get_label_name, get_labels_where},
        con_set_label::{set_label, set_label_name},
    },
    component::com_table::table_label::LabelTable,
};
use dioxus::prelude::*;

pub fn content_label() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut id_show = use_signal(|| 0);
    let mut updated_data = use_signal(|| get_label_name().expect("Failed to load labels"));
    let mut table_ctx =
        use_signal(|| get_labels_where(*id_show.read()).expect("Failed to load labels"));
    //impl FnMut(i32)

    let set_id_show: Callback<i32> = use_callback(move |id: i32| {
        id_show.set(id);
    });

    rsx! {
        div {
            id: "modal",
            class: "modal",
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
                            set_label_name(label_name);
                            show_modal.set(false);
                            updated_data.set(get_label_name().expect("Failed to load labels"));
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
                            set_label(id_label, abb_ctx);
                            show_modal.set(false);
                            table_ctx
                                .set(get_labels_where(*id_show.read()).expect("Failed to load labels"));
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

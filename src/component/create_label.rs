use dioxus::prelude::*;
//#[component]
// pub fn moadl_label() -> Element {
//     rsx!{
//         div {
//             id: "modal",
//             class: "fixed z-50	  place-items-center place-content-center  bg-black bg-opacity-50",
//             visibility: if *show_modal.read() { "visible" } else { "hidden" },
//             div { class: "h-fit bg-white rounded-lg shadow-lg max-w-md w-full p-6",
//                 div {
//                     div { class: " flex items-center justify-between mb-4",
//                         h2 { class: "text-xl font-semibold", {"modal title"} }
//                         button {
//                             class: "text-gray-400 hover:text-gray-600",
//                             onclick: move |_| {
//                                 println!("{}", id_show.read());
//                                 show_modal.set(false);
//                             },
//                             {"btn"}
//                         }
//                     }
//                 }
//                 if *id_show.read() == 0 {
//                     form {
//                         class: "",
//                         onsubmit: move |evt| {
//                             let label_name: String = evt.data.values()["label"].as_value();
//                             insert_label_name(label_name);
//                             show_modal.set(false);
//                             updated_data.set(select_labels_name().expect("Failed to load labels"));
//                         },
//                         div {
//                             div { class: "flex justify-center",
//                                 input {
//                                     class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
//                                     name: "label", // Add name attribute to capture data
//                                     r#type: "text",
//                                     value: "",
//                                 }
//                             }
//                         }
//                         div { class: "flex justify-end mr-2",
//                             input {
//                                 class: "",
//                                 r#type: "submit",
//                                 class: "btnEdit mt-3 mb-3 mr-10",
//                             }
//                         }
//                     }
//                 } else {
//                     form {
//                         class: "",
//                         onsubmit: move |evt| {
//                             let id_label: i32 = evt
//                                 .data
//                                 .values()["id_label"]
//                                 .as_value()
//                                 .parse::<i32>()
//                                 .unwrap_or(0i32);
//                             let abb_ctx: String = evt
//                                 .data
//                                 .values()["abb_ctx"]
//                                 .as_value()
//                                 .parse()
//                                 .unwrap_or("".to_string());
//                             insert_label(id_label, abb_ctx);
//                             show_modal.set(false);
//                             table_ctx
//                                 .set(select_labels_where(*id_show.read()).expect("Failed to load labels"));
//                         },
//                         div {
//                             div { class: "flex justify-center",
//                                 input {
//                                     class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
//                                     name: "id_label", // Add name attribute to capture data
//                                     r#type: "text",
//                                     value: "",
//                                 }
//                             }
//                             div { class: "flex justify-center",
//                                 input {
//                                     class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
//                                     name: "abb_ctx", // Add name attribute to capture data
//                                     r#type: "text",
//                                     value: "",
//                                 }
//                             }
//                         }
//                         div { class: "flex justify-end mr-2",
//                             input {
//                                 class: "",
//                                 r#type: "submit",
//                                 class: "btnEdit mt-3 mb-3 mr-10",
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
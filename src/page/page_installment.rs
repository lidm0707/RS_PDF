use crate::{
    component::datepicker::PickerDiffMonth,
    database::{db_bank::db_select::select_bank, db_select::select_labels_name},
    entity::entity_label::SelectLabelsName, service::date::{add::month_add, date_format::format_date, now::thai_now},
};
use dioxus::prelude::*;
use chrono::prelude::*;
use svg_attributes::end;

pub fn content_installment() -> Element {
    let mut show_modal: Signal<bool> = use_signal(|| false);
    let mut updated_data: Signal<Vec<SelectLabelsName>> =
        use_signal(|| select_labels_name().expect("Failed to load labels"));
    let now = format!("{:02}/{:02}/{}", 1, thai_now().month(), &thai_now().year().to_string()[2..4]);
    let mut stard_date: Signal<String> = use_signal(|| format_date(&now));
    let mut end_date: Signal<String> = use_signal(|| format_date(&now));
    let mut time: Signal<String> = use_signal(|| "1".to_string());
    let mut period: Signal<String> = use_signal(|| "".to_string());
    let mut total: Signal<String> = use_signal(|| "0.00".to_string());

    let mut time_for_payment = use_callback(move |full: (String, String)| {
        match (full.0.parse::<f64>(), full.1.parse::<i32>()) {
            (Ok(price), Ok(time)) if time != 0 => {
                let result = price / time as f64;
                result
            }
            _ => 0.00,
        }
    });

    rsx! {
        div {
            id: "modal",
            visibility: if *show_modal.read() { "visible" } else { "hidden" },
            class: "fixed inset-0  place-items-center place-content-center  bg-black bg-opacity-50",
            div { class: "h-fit bg-white rounded-lg shadow-lg max-w-md w-full p-6",
                div {
                    div { class: " flex items-center justify-between mb-4",
                        h2 { class: "text-xl font-semibold", {"modal title"} }
                        button {
                            class: "text-gray-400 hover:text-gray-600",
                            onclick: move |_| {
                                show_modal.set(false);
                                println!("{:?}", stard_date);
                                println!("{:?}", end_date);
                            },
                            {"btn"}
                        }
                    }
                }
                form {
                    class: "",
                    onsubmit: move |evt| {
                        println!("{:?}", evt);
                        println!("{:?}", "");
                        println!("{:?}", evt.data.values());
                        println!("{:?}", evt.data.values()["note"].as_value());
                        println!("{:?}", evt.data.values()["amount"].as_value());
                        println!("{:?}", evt.data.values()["total"].as_value());
                        println!("{:?}", time.read());
                        let t = time.read().parse::<i32>().unwrap();
                        for time in 0..t {
                            if let Some(value) = evt.data.values().get(&time.to_string()) {
                                println!("{:?}", value.as_value());
                            }
                        }

                   

                        show_modal.set(false);
                        updated_data.set(select_labels_name().expect("Failed to load labels"));
                    },
                    div { class: "",
                        PickerDiffMonth {
                            stard_date,
                            end_date,
                            time,
                            period,
                        }
                    }
                    div {
                        div { class: "flex",
                            label { class: "w-1/6 mr-3", {"TIME"} }
                            div { {time.read().clone()} }
                        }
                    }

                    div {
                        div { class: "flex",
                            label { class: "w-1/6 mr-3", {"PERIOD"} }
                            div { {period.read().clone()} }
                        }
                    }
                    div {
                        div { class: "flex ",
                            label { class: "w-1/6", {"bank"} }
                            select {
                                class: "border-b w-fit mr-2 ml-2 mb-2",
                                onchange: move |evt| {
                                    println!("{} ", evt.value());
                                },
                                {
                                    select_bank()
                                        .unwrap()
                                        .iter()
                                        .map(|x| {
                                            rsx! {
                                                option { value: "{x.id}", "{x.name}" }
                                            }
                                        })
                                }
                            }
                        }
                    }
                    div {
                        div { class: "flex ",
                            label { class: "w-1/6", {"label"} }
                            select {
                                class: "border-b w-fit  mr-2 ml-2 mb-2",
                                onchange: move |evt| {
                                    println!("{} ", evt.value());
                                },
                                {
                                    select_labels_name()
                                        .unwrap()
                                        .iter()
                                        .map(|x| {
                                            rsx! {
                                                option { value: "{x.id}", "{x.label}" }
                                            }
                                        })
                                }
                            }
                        }
                    }
                    div {
                        div { class: "flex ",
                            label { class: "w-1/6", {"note"} }
                            input {
                                class: "border-b w-fit  mr-2 ml-2 mb-2",
                                name: "note", // Add name attribute to capture data
                                r#type: "text",
                                value: "",
                            }
                        }
                    }

                    div {
                        div { class: "flex ",
                            label { class: "w-1/6", {"amount"} }
                            input {
                                class: "border-b w-fit   mr-2 ml-2 mb-2",
                                name: "amount", // Add name attribute to capture data
                                r#type: "text",
                                value: "",
                            }
                        }
                    }
                    div {
                        div { class: "flex ",
                            label { class: "w-1/6", {"total"} }
                            input {
                                oninput: move |evt| {
                                    total.set(evt.value());
                                },
                                class: "border-b w-fit   mr-2 ml-2 mb-2",
                                name: "total", // Add name attribute to capture data
                                r#type: "text",
                                value: "",
                            }
                        }
                    }


                    div { class: if time.read().parse::<i32>().unwrap_or(0) > 0 { "block" } else { "hidden" },
                        {"PALNING PAYMENT"}
                        {
                            let price = time_for_payment
                                .call((total.read().to_string(), time.read().to_string()));
                            let price_clone = price.clone();
                            (0..time.read().parse::<i32>().unwrap_or(0))
                                .map(move |time| {
                                    let date_time = month_add(stard_date.read().as_ref(),&time.to_string());
                                    rsx! {
                                        div {
                                            div { class: "flex ",
                                                label { class: "w-3/6 mr-4", "date: {date_time}" }
                                                input {
                                                    class: "border-b",
                                                    name: "{time.to_string()}",
                                                    value: price_clone.to_string(),
                                                }
                                            }
                                        }
                                    }
                                })
                        }
                    }

                    div {
                        div { class: "flex justify-end mr-2",
                            input {
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
                // LabelTable { updated_data, id_show, table_ctx ,set_id_show }
        }
    }
}



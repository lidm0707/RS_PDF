use chrono::prelude::*;
use dioxus::prelude::*;

use crate::{
    backend::{
        controller::{
            con_date_handle::{
                con_format_date::{get_format_date, get_format_period},
                con_now::get_thai_now,
            },
            con_db::{
                con_get_cash_in::get_cash_in, con_get_cash_out::get_cash_out, con_get_label::get_label_name, con_get_revenue_type::con_get_revenue_type, con_set_cash_in::set_cash_in, con_set_cash_out::set_cash_out
            },
        },
        model::{model_cash_in::ModelCashIn, model_cash_out::ModelCashOut},
    },
    component::{com_date::datepicker::PickerDate, com_table::table_cash::CashTable},
};

pub fn content_cash() -> Element {
    let now = format!(
        "{:02}/{:02}/{}",
        get_thai_now().day(),
        get_thai_now().month(),
        &get_thai_now().year().to_string()[2..4]
    );
    let mut data_table_out: Signal<Vec<ModelCashOut>> = use_signal(|| get_cash_out().unwrap());
    let mut data_table_in: Signal<Vec<ModelCashIn>> = use_signal(|| get_cash_in().unwrap());
    let mut show_modal: Signal<bool> = use_signal(|| false);
    let mut label_id: Signal<i32> = use_signal(|| 1);
    let date_signal: Signal<String> = use_signal(|| get_format_date(&now));
    let period: Signal<String> = use_signal(|| get_format_period(&date_signal.read()));
    let mut selected_type: Signal<String> = use_signal(|| "OUT-COME".to_string());

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
                                show_modal.set(false);
                            },
                            {"btn"}
                        }
                    }
                }
                form {
                    class: "",
                    onsubmit: move |evt| {
                        if *selected_type.read() == "IN-COME".to_string() {
                            println!("date_signal: {:?}", evt.values());
                            let _ = set_cash_in(
                                date_signal.read().clone(),
                                period.read().clone(),
                                label_id.read().clone(),
                                Some(evt.values()["note"].as_value()),
                                match evt.values()["amount"].as_value().to_string().parse::<f64>() {
                                    Ok(amount) => amount,
                                    Err(e) => {
                                        println!("Failed to parse amount: {:?}", e);
                                        0.00f64
                                    }
                                },
                            );
                            data_table_in.set(get_cash_in().unwrap());
                            show_modal.set(false);
                            println!("label_name: {:?}", evt);
                        } else {
                            println!("date_signal: {:?}", evt.values());
                            let _ = set_cash_out(
                                date_signal.read().clone(),
                                period.read().clone(),
                                label_id.read().clone(),
                                Some(evt.values()["note"].as_value()),
                                match evt.values()["amount"].as_value().to_string().parse::<f64>() {
                                    Ok(amount) => amount,
                                    Err(e) => {
                                        println!("Failed to parse amount: {:?}", e);
                                        0.00f64
                                    }
                                },
                            );
                            data_table_out.set(get_cash_out().unwrap());
                            show_modal.set(false);
                            println!("label_name: {:?}", evt);
                        }
                    },
                    div {
                        div { class: "flex ",
                            label { class: "mr-2 w-1/6" }
                            div { class: "w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                PickerDate { date: date_signal, period }
                            }
                        }
                        div { class: "flex ",
                            label { class: "mr-2 w-1/6" }
                            div { {period} }
                        }
                        div { class: "flex ",
                            label { class: "mr-2 w-1/6", {"TYPE"} }
                            select {

                                class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                name: "type", // Add name attribute to capture data
                                onchange: move |evt| {
                                    selected_type.set(evt.value());
                                    println!("Selected type: {}", evt.value());
                                },

                                option { value: "OUT-COME", {"OUT-COME"} }
                                option { value: "IN-COME", {"IN-COME"} }
                            }
                        }
                        div {
                            div { class: "flex ",
                                label { class: "w-1/6 w-1/6", {"label"} }
                                if *selected_type.read() == "IN-COME".to_string() {
                                    select {
                                        class: "border-b w-fit  mr-2 ml-2 mb-2",
                                        onchange: move |evt| {
                                            label_id.set(evt.value().parse::<i32>().unwrap());
                                            println!("{} ", evt.value());
                                        },
                                        {
                                            con_get_revenue_type().unwrap()
                                                .iter()
                                                .map(|re_type| {
                                                    let name_type = &re_type.category;
                                                    let re_type_id = re_type.id;
                                                    rsx! {
                                                        option { value: "{re_type_id}", "{name_type}" }
                                                    }
                                                })
                                        }
                                    }
                                } else {
                                    select {
                                        class: "border-b w-fit  mr-2 ml-2 mb-2",
                                        onchange: move |evt| {
                                            label_id.set(evt.value().parse::<i32>().unwrap());
                                            println!("{} ", evt.value());
                                        },
                                        {
                                            get_label_name()
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
                        }
                        div { class: "flex ",
                            label { class: "mr-2 w-1/6", {"NOTE"} }
                            input {
                                class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                name: "note", // Add name attribute to capture data
                                r#type: "text",
                                value: "",
                            }
                        }
                        div { class: "flex ",
                            label { class: "mr-2 w-1/6", {"AMOUNT"} }
                            input {
                                class: "border border-2 w-fit border-black rounded-md mr-2 ml-2 mb-2",
                                name: "amount", // Add name attribute to capture data
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
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                button {
                    class: "btnEdit mt-3 mb-3 mr-10 ml-3",
                    onclick: move |_| {
                        show_modal.set(true);
                    },
                    {"ADD"}
                }
            }
            CashTable { data_table_out, data_table_in }
        }
    }
}

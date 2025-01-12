// id -> Integer,
// date -> Date,
// period -> Text,
// #[sql_name = "type"]
// type_ -> Text,
// label_id -> Integer,
// amount -> Double,

use dioxus::prelude::*;
use chrono::prelude::*;

use crate::{
    component::{com_date::datepicker::PickerDate, com_table::table_cash::CashTable},
    repo::{
        db_cash::{db_insert::insert_cash, db_select::select_cash},
        db_label::db_select::select_labels_name,
    },
    entity::{entity_cash::SelectCash, entity_credit::SelectCredit}, service::date::{date_format::{format_date, format_period}, now::thai_now},
};

#[derive(PartialEq, Clone, Props)]
pub struct TableRaw {
    pub data: Signal<Vec<SelectCredit>>,
}

pub fn content_cash() -> Element {
    let now = format!(
        "{:02}/{:02}/{}",
        thai_now().day(),
        thai_now().month(),
        &thai_now().year().to_string()[2..4]
    );
    let mut data_table: Signal<Vec<SelectCash>> = use_signal(|| select_cash().unwrap());
    let mut show_modal: Signal<bool> = use_signal(|| false);
    let mut label_id: Signal<i32> = use_signal(|| 1);
    let mut date_signal: Signal<String> = use_signal(|| format_date(&now));
    let mut period: Signal<String> = use_signal(|| format_period(&date_signal.read()));

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
                        println!("date_signal: {:?}", evt.values());
                        insert_cash(
                            date_signal.read().clone(),
                            period.read().clone(),
                            evt.values()["type"].as_value(),
                            label_id.read().clone(),
                            match evt.values()["amount"].as_value().to_string().parse::<f64>() {
                                Ok(amount) => amount,
                                Err(e) => {
                                    println!("Failed to parse amount: {:?}", e);
                                    0.00f64
                                }
                            },
                        );
                        data_table.set(select_cash().unwrap());
                        show_modal.set(false);
                        println!("label_name: {:?}", evt);
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
                                option { value: "OUT-COME", {"OUT-COME"} }
                                option { value: "IN-COME", {"IN-COME"} }
                            }
                        }
                        div {
                            div { class: "flex ",
                                label { class: "w-1/6 w-1/6", {"label"} }
                                select {
                                    class: "border-b w-fit  mr-2 ml-2 mb-2",
                                    onchange: move |evt| {
                                        label_id.set(evt.value().parse::<i32>().unwrap());
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
            CashTable { data_table }
        }
    }
}

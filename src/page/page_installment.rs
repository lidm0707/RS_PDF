use crate::{
    component::{
        date::datepickermonth::PickerDiffMonth,
        table_component::{
            table_installment::TableInstallment, table_installment_items::TableInstallmentItem,
        },
    },
    database::{
        db_bank::db_select::select_bank,
        db_installment::{
            db_insert::{insert_installment, insert_installment_items},
            db_select::{select_installment, select_installment_items_where},
        },
        db_label::db_select::select_labels_name,
    },
    entity::{
        entity_installment::{SelectInstallment, SelectInstallmentItems},
        entity_label::SelectLabelsName,
    },
    service::date::{add::month_add, date_format::format_date, now::thai_now},
};
use chrono::prelude::*;
use dioxus::prelude::*;

pub fn content_installment() -> Element {
    let mut show_modal: Signal<bool> = use_signal(|| false);
    let mut updated_data: Signal<Vec<SelectLabelsName>> =
        use_signal(|| select_labels_name().expect("Failed to load labels"));
    let now = format!(
        "{:02}/{:02}/{}",
        1,
        thai_now().month(),
        &thai_now().year().to_string()[2..4]
    );
    let mut start_date: Signal<String> = use_signal(|| format_date(&now));
    let mut end_date: Signal<String> = use_signal(|| format_date(&now));
    let mut time: Signal<String> = use_signal(|| "1".to_string());
    let mut period: Signal<String> = use_signal(|| "".to_string());
    let mut total: Signal<f64> = use_signal(|| 0.00);
    let mut static_price: Signal<f64> = use_signal(|| 0.00);
    let mut diff = use_signal(|| 0.00);
    let mut id_table: Signal<i32> = use_signal(|| 0);
    let mut df_installment: Signal<Vec<SelectInstallment>> =
        use_signal(|| select_installment().expect("Failed to load labels"));
    let mut df_installment_items: Signal<Vec<SelectInstallmentItems>> = use_signal(|| {
        select_installment_items_where(*id_table.read()).expect("Failed to load labels")
    });

    let time_for_payment =
        move |full: (String, String)| match (full.0.parse::<f64>(), full.1.parse::<i32>()) {
            (Ok(p), Ok(t)) if t != 0 => {
                let result = p / t as f64;
                format!("{:.2}", result).parse::<f64>().unwrap_or(0.00)
            }
            _ => 0.00,
        };

    let mut label_id: Signal<i32> = use_signal(|| 1);
    let mut bank_id: Signal<i32> = use_signal(|| 1);

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
                                println!("{:?}", start_date);
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
                        println!("{:?}", time.read());
                        let master = insert_installment(
                            start_date.clone().read().to_string(),
                            end_date.clone().read().to_string(),
                            time.read().parse::<i32>().unwrap(),
                            evt.data.values()["note"].as_value().to_string(),
                            *label_id.read(),
                            evt.data.values()["amount"].as_value().to_string().parse::<f64>().unwrap(),
                            *total.read(),
                        );
                        println!("{:?}", master);
                        let t = time.read().parse::<i32>().unwrap();
                        for i in 0..t {
                            if let Some(value) = evt.data.values().get(&i.to_string()) {
                                println!("{:?}", value.as_value());
                                let check_item = insert_installment_items(
                                    start_date.read().clone(),
                                    period.read().clone(),
                                    bank_id.read().clone(),
                                    value.as_value().to_string().parse::<f64>().unwrap(),
                                    master.id,
                                );
                                println!("{:?}", check_item);
                            }
                        }
                        show_modal.set(false);
                        updated_data.set(select_labels_name().expect("Failed to load labels"));
                        df_installment.set(select_installment().expect("Failed to load labels"));
                    },
                    div { class: "",
                        PickerDiffMonth {
                            start_date,
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
                                    bank_id.set(evt.value().parse::<i32>().unwrap());
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
                                    total.set(evt.value().parse::<f64>().unwrap_or(0.00));
                                    let price = format!(
                                        "{:.02}",
                                        time_for_payment((total.read().to_string(), time.read().to_string())),
                                    );
                                    static_price.set(price.parse::<f64>().unwrap_or(0.00));
                                    diff.set(
                                        (*static_price.read() * time.read().parse::<f64>().unwrap_or(0.00))
                                            - *total.read(),
                                    );
                                    diff.read();
                                },
                                class: "border-b w-fit   mr-2 ml-2 mb-2",
                                name: "total", // Add name attribute to capture data
                                r#type: "text",
                                value: "",
                            }
                        }
                    }

                    div {
                        div { class: "flex ",
                            label { class: "w-1/6", {"diff: "} }
                            div { "{diff.read()}" }
                        }
                    }


                    div { class: if time.read().parse::<i32>().unwrap_or(0) > 0 { "block" } else { "hidden" },
                        {"PLANING PAYMENT"}
                        {
                            let time_clone = time.read().clone();
                            let in_total = *total.clone().read();
                            let in_time = time_clone.parse::<f64>().unwrap_or(0.00);
                            (0..time_clone.parse::<i32>().unwrap_or(0))
                                .map(move |time| {
                                    let date_time = month_add(
                                        start_date.clone().read().as_ref(),
                                        &time.to_string(),
                                    );
                                    let in_box = in_total / in_time;
                                    let str_box = format!("{:.2}", in_box).parse::<f64>().unwrap_or(0.00);
                                    rsx! {
                                        div {
                                            div { class: "flex ",
                                                label { class: "w-3/6 mr-4", "date: {date_time}" }
                                                input {
                                                    oninput: move |evt| {
                                                        let price = static_price.clone()
                                                            * (time.to_string().parse::<f64>().unwrap_or(0.00) - 1.00);
                                                        let current = evt.value().parse::<f64>().unwrap_or(0.00);
                                                        let sum = format!("{:.2}", price + current).parse::<f64>().unwrap_or(0.00);
                                                        let new_diff = sum - in_total;
                                                        diff.set(new_diff);
                                                    },
                                                    class: "border-b",
                                                    name: "{time.to_string()}",
                                                    initial_value: "{str_box}",
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
            div {

                if *id_table.read() as i32 == 0 {
                    TableInstallment {
                        df_installment,
                        df_installment_items,
                        id_table,
                    }
                } else {
                    TableInstallmentItem {
                        df_installment_items,
                        df_installment,
                        id_table,
                    }
                }

            }
        }
    }
}

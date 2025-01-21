use crate::{
    backend::controller::{
        con_date_handle::{
            con_date_aggr::set_month_add,
            con_format_date::{get_format_date, get_format_period},
            con_now::get_thai_now,
        },
        con_db::{
            con_get_bank::get_bank,
            con_get_installment::{get_installment, get_installment_items_where},
            con_get_label::get_label_name,
            con_set_installment::{set_installment, set_installment_items},
        },
    },
    component::{
        com_date::datepickermonth::PickerDiffMonth,
        com_table::{
            table_installment::TableInstallment, table_installment_items::TableInstallmentItem,
        },
    },
};
use chrono::prelude::*;
use dioxus::prelude::*;

pub fn content_installment() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut updated_data = use_signal(|| get_label_name().expect("Failed to load labels"));
    let now = format!(
        "{:02}/{:02}/{}",
        1,
        get_thai_now().month(),
        &get_thai_now().year().to_string()[2..4]
    );
    let  start_date: Signal<String> = use_signal(|| get_format_date(&now));
    let  end_date: Signal<String> = use_signal(|| get_format_date(&now));
    let  time: Signal<String> = use_signal(|| "1".to_string());
    let  period: Signal<String> = use_signal(|| "".to_string());
    let mut total: Signal<f64> = use_signal(|| 0.00);
    let mut static_price: Signal<f64> = use_signal(|| 0.00);
    let mut diff = use_signal(|| 0.00);
    let  id_table: Signal<i32> = use_signal(|| 0);
    let mut df_installment = use_signal(|| get_installment().expect("Failed to load labels"));
    let  df_installment_items = use_signal(|| {
        get_installment_items_where(*id_table.read()).expect("Failed to load labels")
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
            class: "modal",
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
                        let master = set_installment(
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
                            let new_period = set_month_add(
                                start_date.clone().read().to_string().as_str(),
                                (i).to_string().as_str(),
                            );
                            let new_period = get_format_period(&new_period);
                            if let Some(value) = evt.data.values().get(&i.to_string()) {
                                println!("{:?}", value.as_value());
                                println!("{:?}", new_period);
                                let check_item = set_installment_items(
                                    start_date.read().clone(),
                                    new_period,
                                    bank_id.read().clone(),
                                    value.as_value().to_string().parse::<f64>().unwrap(),
                                    master.id,
                                );
                                println!("{:?}", check_item);
                            }
                        }
                        show_modal.set(false);
                        updated_data.set(get_label_name().expect("Failed to load labels"));
                        df_installment.set(get_installment().expect("Failed to load labels"));
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
                                    get_bank()
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
                                    let date_time = set_month_add(
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

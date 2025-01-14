use chrono::prelude::*;
use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::{
    backend::{
        controller::{
            con_date_handle::con_now::get_thai_now,
            con_db::{con_get_label::get_label_name, con_set_credit::set_credit},
        },
        model::model_pdf::TranformLine,
    },
    component::{com_table::table_upload::UploadTable, upload::BtnUplaod},
};

pub fn content_upload() -> Element {
    let mut files_uploaded = use_signal(|| Vec::<TranformLine>::new());
    // runtime if vec emty and direct index as i[0] is break!!
    let now_thai = get_thai_now();
    let arr_year: Vec<i32> = (now_thai.year() - 5..=now_thai.year() + 5).collect();
    let mut year = use_signal(move || now_thai.year());
    let mut month = use_signal(move || now_thai.month());

    let chang_yy = use_callback(move |evt: Event<FormData>| {
        let tem_yy = evt.value().parse::<i32>().unwrap();
        let _ = year.set(tem_yy);
        let temp: Vec<_> = files_uploaded.read().clone();
        let tran = temp
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut temp = temp[i].clone();
                temp.period = format!("{}-{:02}", year, month);
                temp
            })
            .collect::<Vec<TranformLine>>();
        files_uploaded.set(tran);
    });

    let chang_mm = use_callback(move |evt: Event<FormData>| {
        let tem_m = evt.value().parse::<u32>().unwrap();
        let _ = month.set(tem_m);
        let temp = files_uploaded.read().clone();
        let tran = temp
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut temp = temp[i].clone();
                temp.period = format!("{}-{:02}", year, month);
                temp
            })
            .collect::<Vec<TranformLine>>();
        files_uploaded.set(tran);
    });

    let sum_amount = files_uploaded
        .read()
        .clone()
        .iter()
        .fold(0.0, |acc, x| acc + x.amount);
    let sum_by_gorupby_label = move |label: i32| {
        files_uploaded.read().clone().iter().fold(0.0, |acc, x| {
            if x.label_id as i32 == label {
                acc + x.amount
            } else {
                acc
            }
        })
    };
    let label_name = use_signal(|| get_label_name().unwrap());

    let format_thai = move |number_f64: f64| {
        let whole_part = (number_f64.trunc() as u64).to_formatted_string(&Locale::th);
        let fractional_part = (number_f64.fract() * 100.0).round() as u64;
        format!("{}.{:02}", whole_part, fractional_part)
    };
    rsx! {
        div { class: "content",
            div { class: "summary",
                div { class: "summary-items",
                    div { class: "p-3 text-center",
                        "AMOUNT"
                        div { class: "text-right each-summary-item", {format_thai(sum_amount)} }
                    }
                }
                {
                    label_name
                        .read()
                        .iter()
                        .map(|x| {
                            rsx! {
                                div { class: "summary-items",
                                    div { class: "p-3 text-center",
                                        "{x.label}"
                                        div { class: "text-right each-summary-item", {format_thai(sum_by_gorupby_label(x.id))} }
                                        div { class: "text-right each-summary-item",
                                            {format!("{:.2} %", (sum_by_gorupby_label(x.id) / sum_amount) * 100.0)}
                                        }
                                    }
                                }
                            }
                        })
                }
            }
            div { class: "control",
                BtnUplaod { file_upload: files_uploaded  }
                select {
                    class: "select",
                    onchange: move |evt| {
                        println!("{}", evt.value());
                        let _ = chang_mm.call(evt);
                    },
                    {
                        (1..=12)
                            .map(|m| {
                                if m == *month.read() {
                                    rsx! {
                                        option { selected: true, value: "{m}", "{m}" }
                                    }
                                } else {
                                    rsx! {
                                        option { value: "{m}", "{m}" }
                                    }
                                }
                            })
                    }
                }

                select {
                    class: "select",
                    onchange: move |evt| {
                        println!("{}", evt.value());
                        let _ = chang_yy.call(evt);
                    },
                    value: "{year.read()}",
                    {
                        arr_year
                            .iter()
                            .map(|item_year| {
                                if item_year == &*year.read() {
                                    rsx! {
                                        option { selected: true, value: "{item_year}", "{item_year}" }
                                    }
                                } else {
                                    rsx! {
                                        option { value: "{item_year}", "{item_year}" }
                                    }
                                }
                            })
                    }
                }
                button {
                    class: "btn",
                    onclick: move |_| {
                        {
                            files_uploaded
                                .iter()
                                .for_each(|line| {
                                    let date = line.date.clone();
                                    let ctx = line.ctx.clone();
                                    let amount = line.amount;
                                    let label_id = line.label_id as i32;
                                    let period = line.period.clone();
                                    let payment_type_id = line.payment_type_id as i32;
                                    set_credit(date, ctx, amount, label_id, period, payment_type_id);
                                });
                        }
                        files_uploaded.set(Vec::<TranformLine>::new());
                    },
                    {"SAVE"}
                }
            }
            UploadTable { file_upload: files_uploaded}
        }
    }
}

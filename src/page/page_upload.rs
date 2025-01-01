use chrono::prelude::*;
use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::{
    component::{
        datepicker::Picker,
        table_component::table_upload::UploadTable,
        upload::{BtnUplaod, FileUpload},
    },
    database::db_select::select_labels_name,
    model::model_pdf::TranformLine,
    service::{date::now::thai_now, pdf::check_label::search_labels},
};

pub fn content_upload() -> Element {
    let mut files_uploaded = use_signal(|| Vec::<TranformLine>::new());
    // runtime if vec emty and direct index as i[0] is break!!
    let now_thai = thai_now();
    let default_yymm = use_signal(|| format!("{}-{:02}", now_thai.year(), now_thai.month()));
    let mut chang_yymm = move |evt: Event<FormData>| {
        let temp = files_uploaded.read().clone();
        let tran = temp
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut temp = temp[i].clone();
                temp.period = evt.value();
                temp
            })
            .collect::<Vec<TranformLine>>();
        files_uploaded.set(tran);
    };

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
    let label_name = use_signal(|| select_labels_name().unwrap());

    let format_thai = move |number_f64: f64| {
        let whole_part = (number_f64.trunc() as u64).to_formatted_string(&Locale::th);
        let fractional_part = (sum_amount.fract() * 100.0).round() as u64;
        format!("{}.{:02}", whole_part, fractional_part)
    };
    rsx! {
        div { class: "content",
            div { class: "summary",
                div { class: "summary-items",
                    div {
                        "AMOUNT"
                        div { class: "text-right",
                            {
                                format_thai(sum_amount)
                            }
                        }
                    }
                }
                {
                    label_name
                        .read()
                        .iter()
                        .map(|x| {
                            rsx! {
                                div { class: "summary-items",
                                    "{x.label}"
                                    div { class: "text-right",
                                        {
                                            format_thai(sum_by_gorupby_label(x.id))
                                        }
                                    }
                                }
                            }
                        })
                }
            }
            div { class: "control",
                Picker {}
                BtnUplaod { file_upload: FileUpload { data: files_uploaded } }
                input {
                    value: "{default_yymm.read()}",
                    oninput: move |evt| {
                        chang_yymm(evt);
                    },
                }
                select { class: "select", autocomplete: "on",
                    option { value: "1", "1" }
                    option { value: "2", "2" }
                    option { value: "3", "3" }
                    option { value: "4", "4" }
                    option { value: "5", "5" }
                    option { value: "6", "6" }
                    option { value: "7", "7" }
                    option { value: "8", "8" }
                    option { value: "9", "9" }
                    option { value: "10", "10" }
                    option { value: "11", "11" }
                    option { value: "12", "12" }
                }
            }
            UploadTable { file_upload: FileUpload { data: files_uploaded } }
        }
    }
}

use dioxus::prelude::*;

use crate::backend::{
    controller::con_db::{
        con_get_label::{get_label_name, get_label_name_where},
        con_get_payment::get_payment_type_where,
    },
    model::model_pdf::TranformLine,
};

#[component]
pub fn UploadTable(file_upload: Signal<Vec<TranformLine>>) -> Element {
    rsx! {
        div { class: "table-container",
            table {
                thead {
                    tr {
                        th { {"DATE"} }
                        th { {"CTX"} }
                        th { {"AMOUNT"} }
                        th { {"LABEL_ID"} }
                        th { {"PERIOD"} }
                        th { {"CHANEL"} }
                    }
                }
                tbody {
                    {
                        file_upload
                            .read()
                            .clone()
                            .iter()
                            .enumerate()
                            .map(|(i, raw)| {
                                let r2 = format!("{:.2}", raw.amount);
                                let l_id = raw.label_id as i32;
                                let label_name = get_label_name_where(l_id)
                                    .unwrap()[0]
                                    .label
                                    .clone();
                                rsx! {
                                    tr {
                                        td { "{raw.date}" }
                                        td { "{raw.ctx}" }
                                        td { class: "text-right", "{r2}" }
                                        td {
                                            select {
                                                value: "{label_name}",
                                                onchange: move |evt| {
                                                    println!("{} - {:?}", i, evt.value());
                                                    let mut data = file_upload.write();
                                                    data[i].label_id = evt.value().parse::<i32>().unwrap() as i64;
                                                },
                                                {
                                                    get_label_name()
                                                        .unwrap()
                                                        .iter()
                                                        .map(|x| {
                                                            rsx! {
                                                                option { value: "{x.id}", selected: "{label_name == x.label}", "{x.label}" }
                                                            }
                                                        })
                                                }
                                            }
                                        }
                                        td { "{raw.period}" }
                                        td {
                                            {
                                                let id_input = raw.payment_type_id.clone() as i32;
                                                get_payment_type_where(id_input).unwrap()[0].chanel.clone()
                                            }
                                        }
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}

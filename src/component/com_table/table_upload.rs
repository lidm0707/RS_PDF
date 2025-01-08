use dioxus::prelude::*;

use crate::{
    component::upload::FileUpload,
    repo::{
        db_label::db_select::{select_labels_name, select_labels_name_where},
        db_payment::db_select::select_payment_type_where,
    },
};

#[component]
pub fn UploadTable(file_upload: FileUpload) -> Element {
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
                            .data
                            .read()
                            .clone()
                            .iter()
                            .enumerate()
                            .map(|(i, raw)| {
                                let r2 = format!("{:.2}", raw.amount);
                                let l_id = raw.label_id as i32;
                                let label_name = select_labels_name_where(l_id)
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
                                                    let mut data = file_upload.data.write();
                                                    data[i].label_id = evt.value().parse::<i32>().unwrap() as i64;
                                                },
                                                {
                                                    select_labels_name()
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
                                                select_payment_type_where(id_input).unwrap()[0].chanel.clone()
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

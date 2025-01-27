use dioxus::prelude::*;

use crate::backend::{controller::con_db::{con_get_label::get_label_name_where, con_get_payment::get_payment_type_where}, model::model_credit::ModelCredit};

#[component]
pub fn CreditTable(data_table: Signal<Vec<ModelCredit>>) -> Element {
    rsx! {
        div { class: "table-container",
            table {
                thead {
                    tr {
                        th { {"DATE"} }
                        th { {"CTX"} }
                        th { {"AMOUNT"} }
                        th { {"LABEL"} }
                        th { {"PERIOD"} }
                        th { {"CHANNEL"} }
                    }
                }
                tbody {
                    {
                        &mut data_table
                            .iter()
                            .map(|raw| {
                                let r2 = format!("{:.2}", raw.amount);
                                let l_id = raw.label_id.clone() as i32;
                                let p_id = raw.payment_type_id as i32;

                                let label_name = match get_label_name_where(l_id) {
                                    Ok(labels) => {
                                        if labels.is_empty() {
                                            "Unknown".to_string()
                                        } else {
                                            labels.first().unwrap().label.clone()
                                        }
                                    }
                                    Err(err) => panic!("{}", err),
                                };
                                let channel_name = match get_payment_type_where(p_id) {
                                    Ok(labels) => {
                                        if labels.is_empty() {
                                            "Unknown".to_string()
                                        } else {
                                            labels.first().unwrap().chanel.clone()
                                        }
                                    }
                                    Err(err) => panic!("{}", err),
                                };
                                rsx! {
                                    tr {
                                        td { "{raw.date}" }
                                        td { "{raw.ctx}" }
                                        td { class: "text-right", "{r2}" }
                                        td {"{label_name}"}
                                        td { "{raw.period}" }
                                        td {
                                            "{channel_name}"
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

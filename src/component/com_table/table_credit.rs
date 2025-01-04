use dioxus::prelude::*;

use crate::{database::{db_label::db_select::select_labels_name_where, db_payment::db_select::select_payment_type_where}, entity::entity_credit::SelectCredit};

#[component]
pub fn CreditTable(data_table: Signal<Vec<SelectCredit>>) -> Element {
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
                                rsx! {
                                    tr {
                                        td { "{raw.date}" }
                                        td { "{raw.ctx}" }
                                        td { class: "text-right", "{r2}" }
                                        td {
                                            {
                                                let input_id = raw.label_id.clone() as i32;
                                                select_labels_name_where(input_id).unwrap()[0].label.clone()
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

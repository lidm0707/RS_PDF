use dioxus::prelude::*;

use crate::{database::db_payment::db_select::select_payment_type_where, entity::entity_credit::SelectCredit};

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
                                        td { "{raw.label_id}" }
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

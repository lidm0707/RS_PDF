use dioxus::prelude::*;

use crate::{controller::con_db::con_get_label::get_label_where,  entity::entity_cash::SelectCash};

    // pub id: i32,
    // pub date: String,
    // pub period: String,
    // pub type_cash: String,
    // pub label_id: i32,
    // pub amount: f64,
#[component]
pub fn CashTable(data_table: Signal<Vec<SelectCash>>) -> Element {
    rsx! {
        div { class: "table-container ",
            table {
                thead {
                    tr {
                        th { {"DATE"} }
                        th { {"PERIOD"} }
                        th { {"TYPE"} }
                        th { {"LABEL"} }
                        th { {"AMOUNT"} }
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
                                        td { "{raw.period}" }
                                        td { "{raw.type_cash}" }
                                        td { class: "text-right", "{r2}" }
                                        td {
                                            {
                                                let input_id = raw.label_id.clone() as i32;
                                                get_label_where(input_id).unwrap()[0].label.clone()
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

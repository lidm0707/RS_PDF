
use dioxus::prelude::*;

use crate::backend::{
    controller::con_db::con_get_label::get_label_name_where, model::{model_cash_in::ModelCashIn, model_cash_out::ModelCashOut}, service::sv_data::sv_get_revenue_type::sv_get_revenue_type_where,
};

#[component]
pub fn CashTable(data_table_out: Signal<Vec<ModelCashOut>> ,data_table_in: Signal<Vec<ModelCashIn>>) -> Element {
    rsx! {
        div { class: "flex",
            div { class: "table-container ",
                table {
                    thead {
                        tr {
                            th { {"DATE"} }
                            th { {"PERIOD"} }
                            th { {"LABEL"} }
                            th { {"NOTE"} }
                            th { {"AMOUNT"} }
                        }
                    }
                    tbody {
                        {
                            data_table_in
                                .iter()
                                .map(|raw| {
                                    let r2 = format!("{:.2}", raw.amount);
                                    let note_value = raw.note.clone().unwrap_or("".to_string());
                                    let input_id = raw.revenue_id.clone() as i32;
                                    let type_revenue = sv_get_revenue_type_where(input_id)
                                        .unwrap()[0]
                                        .category
                                        .clone();
                                    rsx! {
                                        
                                            tr {
                                                td { "{raw.date}" }
                                                td { "{raw.period}" }
                                                td { "{type_revenue}" }
                                                td { "{note_value}" }
                                                td { class: "text-right", "{r2}" }
                                            }
                                        
                                    }
                                })
                        }
                    }
                }
            }
            div { class: "table-container ",
                table {
                    thead {
                        tr {
                            th { {"DATE"} }
                            th { {"PERIOD"} }
                            th { {"LABEL"} }
                            th { {"NOTE"} }
                            th { {"AMOUNT"} }
                        }
                    }
                    tbody {
                        {
                            data_table_out
                                .iter()
                                .map(|raw| {
                                    let r2 = format!("{:.2}", raw.amount);
                                    let note_value = raw.note.clone().unwrap_or("".to_string());
                                    let input_id = raw.label_id.clone() as i32;
                                    let label_name = get_label_name_where(input_id)
                                        .unwrap()[0]
                                        .label
                                        .clone();
                                    rsx! {
                                        
                                            tr {
                                                td { "{raw.date}" }
                                                td { "{raw.period}" }
                                                td { "{label_name}" }
                                                td { "{note_value}" }
                                                td { class: "text-right", "{r2}" }
                                            }

                                        
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}

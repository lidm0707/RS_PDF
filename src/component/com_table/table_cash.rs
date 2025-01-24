
use dioxus::prelude::*;

use crate::backend::{
    controller::con_db::con_get_label::get_label_name_where, model::model_cash::ModelCash, service::sv_data::sv_get_revenue_type::sv_get_revenue_type_where,
};

#[component]
pub fn CashTable(data_table: Signal<Vec<ModelCash>>) -> Element {
    rsx! {
        div { class: "flex",
            div { class: "table-container ",
                table {
                    thead {
                        tr {
                            th { {"DATE"} }
                            th { {"PERIOD"} }
                            th { {"TYPE"} }
                            th { {"LABEL"} }
                            th { {"NOTE"} }
                            th { {"AMOUNT"} }
                        }
                    }
                    tbody {
                        {
                            data_table
                                .iter()
                                .map(|raw| {
                                    let r2 = format!("{:.2}", raw.amount);
                                    let note_value = raw.note.clone().unwrap_or("".to_string());
                                    let input_id = raw.label_id.clone() as i32;
                                    rsx! {
                                        tr {
                                            td { "{raw.date}" }
                                            td { "{raw.period}" }
                                            td { "{raw.type_cash}" }
                                            td {
                                                {
                                                    if raw.type_cash == "OUT-COME" {
                                                        get_label_name_where(input_id).unwrap()[0].label.clone()
                                                    } else {
                                                        sv_get_revenue_type_where(input_id).unwrap()[0].category.clone()
                                                    }
                                                }
                                            }
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

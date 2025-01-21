use dioxus::prelude::*;

use crate::backend::{
    controller::con_db::con_get_label::get_label_name_where, model::model_cash::ModelCash,
};

#[component]
pub fn CashTable(data_table: Signal<Vec<ModelCash>>) -> Element {
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
                        data_table
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
                                                get_label_name_where(input_id).unwrap()[0].label.clone()
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

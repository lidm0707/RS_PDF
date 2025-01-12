use dioxus::prelude::*;

use crate::{repo::{db_label::db_select::select_labels_name_where, db_payment::db_select::select_payment_type_where}};

#[component]
pub fn CreditDashboardTable(data_table: Signal<Vec<(String, Vec<Option<f64>>)>>) -> Element {
    let data = data_table.read(); // Read the value of the signal

    rsx! {
        div { class: "table-container h-2/5 mb-10",
            table {
                {
                    data.iter()
                        .enumerate()
                        .map(|(index, (month, row_data))| {
                            rsx! {
                                if index == 0 {
                                    thead { class: "text-xs rounded-r-lg rounded-l-lg sticky top-0 px-6 py-3" }
                                    {
                                        row_data
                                            .iter()
                                            .map(|data| {
                                                let string_label = select_labels_name_where(data.unwrap() as i32)
                                                    .unwrap()
                                                    .first()
                                                    .map(|label| label.label.clone())
                                                    .unwrap_or_else(|| "UNKNOW".to_string());
                                                let cell_value = string_label;
                                                rsx! {
                                                    td { "{cell_value}" }
                                                }
                                            })
                                    }
                                }
                                tbody {
                                    tr { class: "",
                                        td { "{month}" } // Display the month
                                        {
                                            row_data
                                                .iter()
                                                .map(|data| {
                                                    let pure = data.unwrap_or(0.00f64);
                                                    let cell_value = pure.to_string();
                                                    rsx! {
                                                        td { "{cell_value}" }
                                                    }
                                                })
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
use dioxus::prelude::*;

use crate::{database::{db_label::db_select::select_labels_name_where, db_payment::db_select::select_payment_type_where}};

#[component]
pub fn CreditDashboardTable(data_table: Signal<Vec<(String, Vec<Option<f64>>)>>) -> Element {
    let data = data_table.read(); // Read the value of the signal

    rsx! {
        div { class: "table-container",
            table {
                thead {
                }
                tbody {
                    {
                        data.iter()
                            .map(|(month, row_data)| {
                                rsx! {
                                    tr {    
                                        td { "{month}" } // Display the month
                                        {
                                            row_data
                                                .iter()
                                                .map(|data| {
                                                    let pure = data.unwrap_or(0.00f64);
                                                    rsx! {
                                                        td { "{pure}" }
                                                    }
                                                })
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
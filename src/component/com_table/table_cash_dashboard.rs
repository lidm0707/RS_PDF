use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn CashDashboardTable(data_table: Signal<Vec<(String, Vec<Option<f64>>)>>) -> Element {
    let data = data_table.read(); // Read the value of the signal

    rsx! {
        div { class: "table-container h-1/5",
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

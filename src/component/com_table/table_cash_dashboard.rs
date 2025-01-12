use dioxus::prelude::*;
use std::collections::HashMap;

use crate::repo::db_label::db_select::select_labels_name_where;

#[component]
pub fn CashDashboardTable(data_table: Signal<Vec<(String, Vec<Option<f64>>)>>) -> Element {
    let data = data_table.read(); // Read the value of the signal

    rsx! {
        div { class: "table-container h-2/5",
            table {
                thead {
                    // Define the table headers (optional if needed)
                }
                tbody {
                    {
                        data.iter()
                            .enumerate()
                            .map(|(index, (month, row_data))| {
                                rsx! {
                                    tr {
                                        // Apply the style to the first row
                                        class: if index == 0 { 
                                            "text-xs rounded-r-lg rounded-l-lg sticky top-0 px-6 py-3" 
                                        } else if index == data.len() - 1 { 
                                            "text-xs rounded-r-lg rounded-l-lg sticky bottom-0 px-6 py-3" 
                                        } else { 
                                            "".to_string() 
                                        },
                                        td { "{month}" } // Display the month
                                        {
                                            row_data.iter().map(|data| {
                                                let pure = data.unwrap_or(0.00f64);
                                                
                                                // Initialize string_label as an empty string and change it if it's the first row
                                                let string_label = if index == 0 {
                                                    select_labels_name_where(data.unwrap() as i32)
                                                        .unwrap()
                                                        .first()
                                                        .map(|label| label.label.clone())
                                                        .unwrap_or_else(|| "CREDIT".to_string())
                                                } else {
                                                    "".to_string() // Empty for non-first rows
                                                };

                                                // Calculate the value to render before passing it to rsx
                                                let cell_value = if index == 0 {
                                                    string_label
                                                } else {
                                                    pure.to_string()
                                                };

                                                // Render the calculated cell value
                                                rsx! {
                                                    td { "{cell_value}" }
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

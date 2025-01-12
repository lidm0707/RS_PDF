use dioxus::prelude::*;

#[component]
pub fn NetDashboardTable(data_table: Signal<Vec<(String, f64, f64)>>) -> Element {
    let data = data_table.read(); // Read the value of the signal

    rsx! {
        div { class: "table-container h-2/5",
            table {
                {
                    data.iter()
                        .enumerate()
                        .map(|(index, (month, re, ex))| {
                            rsx! {
                                if index == 0 {
                                    thead { class: "text-xs rounded-r-lg rounded-l-lg sticky top-0 px-6 py-3",
                                        tr {
                                            td { "period" }
                                            td { "REVERNUE" }
                                            td { "EXPENESE" }
                                            td { "NET" }
                                        }
                                    }
                                } else {
                                    tbody {
                                        tr {
                                            // Apply the style to the first row
                                            class: "",
                                            td { "{month}" } // Display the month
                                            td { "{re}" }
                                            td { "{ex}" }
                                            td { class: if re - ex > 0.0 { "text-green-500" } else { "text-red-500" } ,"{re - ex}"}
                                            
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

use dioxus::prelude::*;

#[component]
pub fn NetDashboardTable(data_table: Signal<Vec<(String, (f64,f64,f64), f64)>>) -> Element {
    let data = data_table.read(); // Read the value of the signal

    rsx! {
        div { class: "table-container h-2/5",
            table {
                {
                    data.iter()
                        .enumerate()
                        .map(|(index, (month, all_re, ex))| {
                            let re = all_re.0;
                            let extra = all_re.1;
                            let refund = all_re.2;
                            let total = re + extra + refund;
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
                                }
                                tbody {
                                    tr {
                                        // Apply the style to the first row
                                        class: "",
                                        td { "{month}" } // Display the month
                                        td {
                                            div { "revenue: {re}" }
                                            div { "extra: {extra}" }
                                            div { "refund: {refund}" }
                                            div { "total: {total}" }
                                        }
                                        td { "{ex}" }
                                        td { class: if re - ex > 0.0 { "text-green-500" } else { "text-red-500" }, "{total - ex}" }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}

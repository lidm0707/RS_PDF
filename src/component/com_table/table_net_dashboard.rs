use dioxus::prelude::*;

use crate::{
    backend::controller::con_db::con_get_plan::get_plan_by_period, format::format_with_separator,
};

#[component]
pub fn NetDashboardTable(data_table: Signal<Vec<(String, (f64, f64, f64), f64)>>) -> Element {
    rsx! {
        div { class: "table-container h-2/5",
            table {
                {
                    data_table
                        .read()
                        .iter()
                        .enumerate()
                        .map(|(index, (month, all_re, ex))| {
                            let re = all_re.0;
                            let extra = all_re.1;
                            let refund = all_re.2;
                            let total = re + extra + refund;
                            let plan = get_plan_by_period(&month).1;
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
                                        td { class: "w-25","{month}" } // Display the month
                                        td { class: "w-40",
                                            div { class: "flex",
                                                div { class: "flex justify-start mr-2 w-10",
                                                    div { class: "justify-items-start ",
                                                        div { "revenue: " }
                                                        div { "extra: " }
                                                        div { "refund: " }
                                                        div { "total: " }
                                                    }
                                                }
                                                div { class: "flex justify-end w-30",
                                                    div { class: "justify-items-end ",
                                                        div { "{format_with_separator(&re)}" }
                                                        div { "{format_with_separator(&extra)}" }
                                                        div { "{format_with_separator(&refund)}" }
                                                        div { "{format_with_separator(&total)}" }
                                                    }
                                                }
                                            }
                                        }
                                        td { class: "w-30",
                                            div { class: "flex",
                                                div { class: "flex justify-start mr-2 w-10",
                                                    div { class: "justify-items-start ",
                                                        div { "ex: " }
                                                        div { "plan: " }
                                                    }
                                                }
                                                div { class: "flex justify-end w-30",
                                                    div { class: "justify-items-end ",
                                                        div { "{format_with_separator(&ex)}" }
                                                        div { "{format_with_separator(&plan)}" }
                                                    }
                                                }
                                            }
                                        }
                                        td { class: "w-30",
                                            div { class: "flex",
                                                div { class: "flex justify-start mr-2 w-10",
                                                    div { class: "justify-items-start ",
                                                        div { "actual: " }
                                                        div { "plan: " }
                                                    }
                                                }
                                                div { class: if re - ex > 0.0 { "flex justify-end text-green-500 w-30" } else { "flex justify-end text-red-500 w-30" },
                                                    div { class: "justify-items-end ",
                                                        div { "{format_with_separator(&(total - ex))}" }
                                                        div { "{format_with_separator(&(total - plan))}" }
                                                    }
                                                }
                                            }
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

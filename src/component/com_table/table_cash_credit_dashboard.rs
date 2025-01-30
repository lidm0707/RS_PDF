use dioxus::prelude::*;
use std::{collections::HashMap, rc::Rc};

use crate::{
    backend::controller::con_db::{
        con_get_label::get_label_name, con_get_plan::get_plan_where,
        con_set_plan_credit::set_plan_credit,
    },
    format::format_with_separator,
};

#[component]
pub fn CashCreditDashboardTable(
    data_table: Signal<Vec<(String, HashMap<i32, HashMap<String, f64>>)>>,
    editing_cells: Signal<Vec<String>>,
    extend:Signal<bool>,
) -> Element {
    // Retrieve label names, handling potential errors
    let label_name = match get_label_name() {
        Ok(labels) => labels,
        Err(_) => vec![], // Fallback to an empty vector if labels cannot be retrieved
    };
    rsx! {
        div { class: "table-container h-2/5 mb-10",
            table {
                thead {
                    tr {
                        td { "Period" } // Add a header for the period column
                        {label_name.iter().map(|data| rsx! {
                            td { "{data.label}" }
                        })}
                    }
                }
                tbody {
                    {
                        data_table
                            .read()
                            .clone()
                            .iter()
                            .map(|(period, label_map)| {
                                rsx! {
                                    tr {
                                        td {
                                            div { class: "flex w-20",
                                                div { class: " w-15", "{period}" }
                                                div { class: "flex justify-start w-5",
                                                    div { class: "justify-items-start ",
                                                        div { class: if *extend.read() { "" } else { "hidden" }, "CR" }
                                                        div { class: if *extend.read() { "" } else { "hidden" }, "CA" }
                                                        div { "TO" }
                                                        div { "PL" }
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            label_name
                                                .iter()
                                                .map(|data| {
                                                    let period_clone = period.clone();
                                                    let period_onchange = period.clone();
                                                    let data_id_onchange = data.id.clone();
                                                    let main_data = label_map.get(&data.id).unwrap();
                                                    let cash = main_data.get("cash").unwrap_or(&0.00);
                                                    let credit = main_data.get("credit").unwrap_or(&0.00);
                                                    let total = cash + credit;
                                                    let plan = get_plan_where(&period_clone, data.id);
                                                    let format_id = format!("{}```{}", period_clone, data.id);
                                                    let rc_format_id = Rc::new(format_id);
                                                    let rc_format_id_1 = Rc::clone(&rc_format_id);
                                                    let rc_format_id_2 = Rc::clone(&rc_format_id);
                                                    let is_editing = editing_cells
                                                        .read()
                                                        .contains(&Rc::clone(&rc_format_id).to_string());
                                                    let plan_value = get_plan_where(&period_clone, data.id);
                                                    rsx! {
                                                        td {
                                                            onclick: move |_| {
                                                                editing_cells.write().clear();
                                                                editing_cells.write().push(rc_format_id_1.to_string());
                                                                println!("{:?}", editing_cells);
                                                            },
                                                            div { class: "flex  w-full mr-2",
                                                                div { class: "flex justify-end w-20",
                                                                    div { class: "justify-items-end",
                                                                        div { class: if *extend.read() { "" } else { "hidden" }, "{format_with_separator(&credit)}" }
                                                                        div { class: if *extend.read() { "" } else { "hidden" }, "{format_with_separator(&cash)}" }
                                                                        div { "{format_with_separator(&total)}" }
                                                                        div {
                                                                            if !is_editing {
                                                                                "{format_with_separator(&plan)}"
                                                                            } else {
                                                                                input {
                                                                                    class: "justify-items-end w-full",
                                                                                    onchange: move |evt| {
                                                                                        let format_onchange = rc_format_id_2.to_string();
                                                                                        editing_cells.write().retain(|cell| *cell != (format_onchange));
                                                                                        let parsed_value = evt.value().clone().parse::<f64>().unwrap_or(0.0);
                                                                                        let formatted_value = format!("{:.2}", parsed_value);
                                                                                        set_plan_credit(
                                                                                            period_onchange.to_string(),
                                                                                            data_id_onchange.clone(),
                                                                                            formatted_value.parse::<f64>().unwrap(),
                                                                                        );
                                                                                    },
                                                                                    value: format_with_separator(&plan_value), // Ensure the value is properly formatted
                                                                                    initial_value: format_with_separator(&plan_value), // Ensure the initial value is properly formatted
                                                                                }
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
                            })
                    }
                }
            }
        }
    }
}

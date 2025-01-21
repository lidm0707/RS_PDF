use dioxus::prelude::*;
use std::{collections::HashMap, rc::Rc};

use crate::backend::controller::con_db::{
    con_get_label::get_label_name, con_get_plan_credit::get_plan_credit_where,
    con_set_plan_credit::set_plan_credit,
};

#[component]
pub fn CashCreditDashboardTable(
    data_table: Signal<Vec<(String, HashMap<i32, HashMap<String, f64>>)>>,
) -> Element {
    // Retrieve label names, handling potential errors
    let label_name = match get_label_name() {
        Ok(labels) => labels,
        Err(_) => vec![], // Fallback to an empty vector if labels cannot be retrieved
    };
    let mut editing_cells = use_signal(|| Vec::<String>::new());
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
                                        td { "{period}" }
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
                                                    let plan = get_plan_credit_where(&period_clone, data.id);
                                                    let format_id = format!("{}```{}", period_clone, data.id);
                                                    let rc_format_id = Rc::new(format_id);
                                                    let rc_format_id_1 = Rc::clone(&rc_format_id);
                                                    let rc_format_id_2 = Rc::clone(&rc_format_id);
                                                    let is_editing = editing_cells.read().contains(&Rc::clone(&rc_format_id).to_string());
                                                    let plan_value = get_plan_credit_where(&period_clone, data.id);
                                                    rsx! {
                                                        td {
                                                            class: " w-fit",
                                                            onclick: move |_| {
                                                                editing_cells.write().clear();
                                                                editing_cells.write().push(rc_format_id_1.to_string());
                                                                println!("{:?}", editing_cells);
                                                            },
                                                            div { class: "flex",
                                                                div {
                                                                    div { class: "flex justify-start mr-2",
                                                                        div {
                                                                            div { "CR" }
                                                                            div { "CA" }
                                                                            div { "TO" }
                                                                            div { "PL" }
                                                                        }
                                                                    }
                                                                }
                                                                div { class: "flex justify-end w-full",
                                                                    div { class:"justify-items-end",
                                                                        div { "{credit}" }
                                                                        div { "{cash}" }
                                                                        div { "{total}" }
                                                                        if !is_editing {
                                                                            div {
                                                                                div { "{plan}" }
                                                                            }
                                                                        } else {
                                                                            div {class:"justify-items-end",
                                                                                input { class:"w-24",
                                                                                    onchange: move |evt| {
                                                                                        let format_onchange = rc_format_id_2.to_string();
                                                                                        editing_cells.write().retain(|cell| *cell != (format_onchange));
                                                                                        set_plan_credit(
                                                                                            period_onchange.to_string(),
                                                                                            data_id_onchange.clone(),
                                                                                            evt.value().clone().parse::<f64>().unwrap(),
                                                                                        );
                                                                                    },
                                                                                    value: "{plan_value}",
                                                                                    initial_value: "{plan_value}",
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

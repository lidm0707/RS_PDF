use dioxus::prelude::*;

use crate::backend::controller::con_db::{
    con_get_label::get_label_name_where, con_get_plan_credit::get_plan_credit_where,
    con_set_plan_credit::set_plan_credit,
};
#[component]
pub fn CreditDashboardTable(data_table: Signal<Vec<(String, Vec<Option<f64>>)>>) -> Element {
    let mut arr_label: Signal<Vec<i32>> = use_signal(|| Vec::<i32>::new());
    let mut editing_cells = use_signal(|| Vec::<(String, i32)>::new()); // Track editing cells (month, index)

    {
        let data_table = data_table.read().clone();
        use_effect(move || {
            let mut new_labels = Vec::new();
            for (i, (_, row_data)) in data_table.iter().enumerate() {
                if i == 0 {
                    for temp_data in row_data.iter() {
                        if let Some(value) = temp_data {
                            new_labels.push(*value as i32);
                        }
                    }
                }
            }
            arr_label.write().extend(new_labels);
            println!("{}", "useeffect");
        });
    }

    rsx! {
        div { class: "table-container h-2/5 mb-10",
            table {
                {
                    data_table
                        .read()
                        .iter()
                        .enumerate()
                        .map(|(index, (month, row_data))| {
                            rsx! {
                                if index == 0 {
                                    thead { class: "text-xs rounded-r-lg rounded-l-lg sticky top-0 px-6 py-3",
                                        tr {
                                            td { "{month}" }
                                            {
                                                row_data
                                                    .iter()
                                                    .map(|temp_data| {
                                                        let string_label = get_label_name_where(temp_data.unwrap() as i32)
                                                            .unwrap()
                                                            .first()
                                                            .map(|temp_label| temp_label.label.clone())
                                                            .unwrap_or_else(|| "UNKNOW".to_string());
                                                        rsx! {
                                                            td { "{string_label}" }
                                                        }
                                                    })
                                            }
                                        }
                                    }
                                } else {
                                    tbody {
                                        tr { class: "",
                                            td { "{month}" }
                                            {
                                                row_data
                                                    .iter()
                                                    .enumerate()
                                                    .map(|(index, temp_data)| {
                                                        let pure = temp_data.unwrap_or(0.00f64);
                                                        let l_id = arr_label.read().get(index).copied().unwrap_or_default();
                                                        let new_id = format!("{}````{}", month, l_id);
                                                        let is_editing = editing_cells.read().contains(&(new_id.clone(), l_id));
                                                        rsx! {
                                                            td {
                                                                div {
                                                                    id: "{new_id}",
                                                                    onclick: move |_| {
                                                                        editing_cells.write().clear();
                                                                        editing_cells.write().push((new_id.clone(), l_id));
                                                                        
                                                                        println!("{:?}", editing_cells);
                                                                        println!("{:?}", arr_label);
                                                                    },
                                                                    div { "{pure}" }
                                                                    {
                                                                        let month = month.clone();
                                                                        let mut plan_value = get_plan_credit_where(&month, l_id);
                                                                        println!("{:?}", plan_value);
                                                                        let new_id_in = new_id.clone();
                                                                        rsx! {
                                                                            if is_editing {
                                                                                input {
                                                                                    class: "border text-xs px-2 py-1",
                                                                                    onchange: move |evt| {
                                                                                        println!("{}", evt.value());
                                                                                        println!("{:?}", editing_cells);
                                                                                        editing_cells.write().retain(|cell| *cell != (new_id_in.clone(), l_id));
                                                                                        set_plan_credit(
                                                                                            month.to_string(),
                                                                                            l_id.clone(),
                                                                                            evt.value().clone().parse::<f64>().unwrap(),
                                                                                        );
                                                                                        let get_value = get_plan_credit_where(&month, l_id);
                                                                                        plan_value = get_value;
                                                                                    },
                                                                                    value: "{plan_value}",
                                                                                }
                                                                            } else {
                                                                                div { class: "flex",
                                                                                    div { class: "text-xs/6 text-stone-300 mr-3", "{plan_value}" }
                                                                                    div { class: if plan_value - pure == 0.0 { "text-xs/6 text-stone-300" } else { "text-red-500" },
                                                                                        {
                                                                                            let mut per = 0.00;
                                                                                            if plan_value != 0.00 {
                                                                                                let cal = ((plan_value - pure) / plan_value) * 100.00;
                                                                                                per = format!("{:.02}", cal).parse::<f64>().unwrap();
                                                                                            }
                                                                                            rsx! {
                                                                                            "{per}%"
                                                                                            }
                                                                                        }
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
                                }
                            }
                        })
                }
            }
        }
    }
}

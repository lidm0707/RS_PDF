

use dioxus::prelude::*;

use crate::backend::controller::con_date_handle::{con_date_aggr::set_date_diff, con_format_date::get_format_period};



#[component]
pub fn PickerDiffMonth(start_date:Signal<String>,end_date:Signal<String>,time:Signal<String>,period:Signal<String>,) ->Element {
    //start_date_date,start_date_date
    rsx!{
        div { class: "flex p-1",
            label { class: "content-center mr-2", {"Start"} }
            input {
                class: "border border-gray-300 rounded-lg p-2 w-64 focus:ring-2 focus:ring-blue-500 focus:outline-none mr-2",
                r#type: "date",
                value: start_date.read().to_string(),
                onchange: move |event| {
                    start_date.set(event.value());
                    println!("{:?}", start_date.read());
                    match set_date_diff(&start_date.read().clone(), &end_date.read().clone()) {
                        Ok(diff) => {
                            period.set(get_format_period(&start_date.read()));
                            time.set(diff.to_string())
                        }
                        Err(_) => println!("Error calculating diff"),
                    }
                },
            }
            label { class: "content-center mr-2", {"End"} }
            input {
                class: "border border-gray-300 rounded-lg p-2 w-64 focus:ring-2 focus:ring-blue-500 focus:outline-none ",
                r#type: "date",
                initial_value: end_date.read().to_string(),
                onchange: move |event| {
                    end_date.set(event.value());
                    println!("{:?}", end_date.read());
                    match set_date_diff(&start_date.read().clone(), &end_date.read().clone()) {
                        Ok(diff) => {
                            period.set(get_format_period(&start_date.read()));
                            time.set(diff.to_string())
                        }
                        Err(err) => println!("Error calculating diff: {:?}", err),
                    }
                },
                // Set default date
                onmounted: move |_| {
                    if end_date.read().is_empty() {
                        end_date.set("2023-01-01".to_string());
                    }
                },
            }    }
    }
   
}



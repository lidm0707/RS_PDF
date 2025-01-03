

use dioxus::prelude::*;

use crate::service::date::{date_format::format_period, diff::diff_month};


#[component]
pub fn PickerDiffMonth(stard_date:Signal<String>,end_date:Signal<String>,time:Signal<String>,period:Signal<String>,) ->Element {
    //stard_date_date,stard_date_date
    rsx!{
        div { class: "flex p-1",
            label { class: "content-center mr-2", {"Start"} }
            input {
                class: "border border-gray-300 rounded-lg p-2 w-64 focus:ring-2 focus:ring-blue-500 focus:outline-none mr-2",
                r#type: "date",
                value: stard_date.read().to_string(),
                onchange: move |event| {
                    stard_date.set(event.value());
                    println!("{:?}", stard_date.read());
                    match diff_month(stard_date.read().clone(), end_date.read().clone()) {
                        Ok(diff) => {
                            period.set(format_period(&stard_date.read()));
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
                    match diff_month(stard_date.read().clone(), end_date.read().clone()) {
                        Ok(diff) => {
                            period.set(format_period(&stard_date.read()));
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



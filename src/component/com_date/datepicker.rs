

use dioxus::prelude::*;

use crate::backend::controller::con_date_handle::con_format_date::get_format_period;




#[component]
pub fn PickerDate(date:Signal<String>,period:Signal<String>) ->Element {
    //date_date,date_date
    rsx!{
        div { class: "flex p-1",
            label { class: "content-center mr-2", {"Start"} }
            input {
                class: "border border-gray-300 rounded-lg p-2 w-64 focus:ring-2 focus:ring-blue-500 focus:outline-none mr-2",
                r#type: "date",
                value: date.read().to_string(),
                onchange: move |event| {
                    date.set(event.value());
                    period.set(get_format_period(&date.read()));
                },
            }
        }
    }
   
}





use dioxus::prelude::*;


#[component]
pub fn Picker() -> Element {
    let mut stard = use_signal( || String::from(""));
    let mut end = use_signal( || String::from(""));
    rsx!{

        div { class: "flex p-4 bg-white rounded-lg shadow-md",
            input {
                class: "border border-gray-300 rounded-lg p-2 w-64 focus:ring-2 focus:ring-blue-500 focus:outline-none mr-11",
                r#type: "date",
                value: "{stard}",
                oninput: move |event| {
                    stard.set(event.value());
                    println!("{:?}", stard.read());
                },
            }
            input {
                class: "border border-gray-300 rounded-lg p-2 w-64 focus:ring-2 focus:ring-blue-500 focus:outline-none ",
                r#type: "date",
                value: "{end}",
                oninput: move |event| {
                    end.set(event.value());
                    println!("{:?}", end.read());
                },
            }
        }
    }
   
}



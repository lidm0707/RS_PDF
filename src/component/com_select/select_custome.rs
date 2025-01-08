use dioxus::prelude::*;


#[component]
pub fn select_onchang(data:Signal<String>,option:Element) -> Element{

        rsx!{
            select {
                class: "select",
                onchange: move |evt| {
                    println!("{}", evt.value());
                    let _ = data.set(evt.value().to_string());
                },
                {option}
            }
        }
    

    }
    

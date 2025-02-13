use dioxus::prelude::*;

use crate::{backend::controller::con_db::con_get_credit::get_credit, component:: com_table::table_credit::CreditTable};

#[component]
pub fn content_credit() -> Element {
    let data_table= use_signal(|| get_credit().unwrap());

    rsx! {
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                // Picker{}

            }

            CreditTable{ data_table }
        }
    }
}

use dioxus::prelude::*;

use crate::{
    component:: table_component::table_credit::CreditTable, database::db_credit::db_select::select_credit, entity::entity_credit::SelectCredit
};

#[derive(PartialEq, Clone, Props)]
pub struct TableRaw {
    pub data: Signal<Vec<SelectCredit>>,
}

pub fn content_credit() -> Element {
    let data_table: Signal<Vec<SelectCredit>> = use_signal(|| select_credit().unwrap());

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

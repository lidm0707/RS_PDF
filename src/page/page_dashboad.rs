use dioxus::prelude::*;

use crate::{
    component:: com_table::{ table_credit_dashboard::CreditDashboardTable}, controller::con_dashboard::con_dash_credit::get_dashboard_credit, database::db_credit::db_select::select_credit, entity::entity_credit::SelectCredit
};

#[derive(PartialEq, Clone, Props)]
pub struct TableRaw {
    pub data: Signal<Vec<SelectCredit>>,
}

pub fn content_dashboard_credit() -> Element {
    let mut start = use_signal(|| "2025-01".to_string());
    let mut end = use_signal(|| "2025-01".to_string());
    let mut data_table: Signal<Vec<(String, Vec<Option<f64>>)>> = use_signal(|| get_dashboard_credit(start.read().as_ref(),end.read().as_ref()).unwrap());
    

    rsx! {
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                // Picker{}

            }

            CreditDashboardTable{ data_table }
        }
    }
}

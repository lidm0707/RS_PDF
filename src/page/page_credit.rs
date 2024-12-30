use dioxus::prelude::*;

use crate::{component::{datepicker::Picker, table::RawTable}, database::db_select::select_raws, entity::entity_credit::SelectCredit};

#[derive(PartialEq, Clone, Props)]
pub struct TableRaw {
   pub  data:Signal<Vec<SelectCredit>>,
}


pub fn content_credit() ->Element{
    let table_raw: Signal<Vec<SelectCredit>> = use_signal(|| select_raws().unwrap());

    rsx!{
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                Picker{}
            
            }
            RawTable{ data_table: table_raw }
        }
    }

}
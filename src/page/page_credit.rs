use dioxus::prelude::*;

use crate::{component::{datepicker::Picker, table::RawTable}, database::db_select::select_raws, models::model_raw::SelectRaws};

#[derive(PartialEq, Clone, Props)]
pub struct TableRaw {
   pub  data:Signal<Vec<SelectRaws>>,
}


pub fn content_credit() ->Element{
    let table_raw: Signal<Vec<SelectRaws>> = use_signal(|| select_raws().unwrap());

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
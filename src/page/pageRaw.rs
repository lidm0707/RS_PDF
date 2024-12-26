use dioxus::prelude::*;

use crate::{component::{datepicker::Picker, table::RawTable}, models::model_raw::SelectRaws};

#[derive(PartialEq, Clone, Props)]
pub struct TableRaw {
   pub  data:Signal<Vec<SelectRaws>>,
}


pub fn PageRaw(table_raw:TableRaw) ->Element{
    rsx!{
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                Picker{}
            
            }
            RawTable{ data_table: table_raw.data }
        }
    }

}
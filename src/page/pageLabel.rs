use dioxus::prelude::*;

use crate::{component::{datepicker::Picker, table::LabelTable}, models::model_label::SelectLabels};

#[derive(PartialEq, Clone, Props)]
pub struct Tablelabel {
    pub data:Signal<Vec<SelectLabels>>,
}

pub fn PageLabel(table_label: Tablelabel) ->Element{
    rsx!{
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                Picker{}
                
            }
            LabelTable { data_table: table_label.data }
        }
    }

}
use dioxus::prelude::*;

use crate::component::{datepicker::Picker, table::UploadTable, upload::{BtnUplaod, FileUpload}};

#[derive(PartialEq, Clone, Props)]
pub struct TableUpload {
  pub data:Signal<Vec<(String, String, f64)>>,
}


pub fn PageUpload(table_upload:TableUpload) ->Element{
    rsx!{
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                Picker{}
                BtnUplaod{file_upload:FileUpload { data: table_upload.data }}
            }
            UploadTable{files_uploaded:table_upload.data.read().to_vec()}
        }
    }

}
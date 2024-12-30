use dioxus::prelude::*;

use crate::component::{datepicker::Picker, table::UploadTable, upload::{BtnUplaod, FileUpload}};

#[derive(PartialEq, Clone, Props)]
pub struct TableUpload {
  pub data:Signal<Vec<(String, String, f64)>>,
}


pub fn content_upload() ->Element{
    let  files_uploaded = use_signal(|| Vec::<(String, String, f64)>::new() );

    rsx!{
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                Picker{}
                BtnUplaod{file_upload:FileUpload { data: files_uploaded }}
            }
            UploadTable{files_uploaded:files_uploaded.read().to_vec()}
        }
    }

}
use dioxus::prelude::*;

use crate::{component::{
    datepicker::Picker, table_component::table_upload::UploadTable, upload::{BtnUplaod, FileUpload}
}, model::model_pdf::TranformLine};



pub fn content_upload() -> Element {
    let files_uploaded = use_signal(|| Vec::<TranformLine>::new());

    rsx! {
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                Picker {}
                BtnUplaod { file_upload: FileUpload { data: files_uploaded } }
            }
            UploadTable{ file_upload: FileUpload { data: files_uploaded } }
        }
    }
}

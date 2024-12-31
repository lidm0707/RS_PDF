use dioxus::prelude::*;
use chrono::prelude::*;

use crate::{component::{
    datepicker::Picker, table_component::table_upload::UploadTable, upload::{BtnUplaod, FileUpload}
}, model::model_pdf::TranformLine, service::date::now::thai_now};



pub fn content_upload() -> Element {
    let files_uploaded = use_signal(|| Vec::<TranformLine>::new());
    // runtime if vec emty and direct index as i[0] is break!!
    let now_thai =thai_now();
    let default_yymm = use_signal(|| format!("{}-{}", now_thai.year(), now_thai.month()));

    rsx! {
        div { class: "content",
            div { class: "summary" }
            div { class: "control",
                Picker {}
                BtnUplaod { file_upload: FileUpload { data: files_uploaded } }
                input { value: "{default_yymm.read()}"}
            }
            UploadTable{ file_upload: FileUpload { data: files_uploaded } }
        }
    }
}

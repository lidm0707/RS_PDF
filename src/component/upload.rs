use crate::service::pdf::read_pdf::read_credit_kbank;
use dioxus::prelude::*;
use dioxus_elements::FileEngine;
use std::sync::Arc;

#[derive(PartialEq, Clone, Props)]
pub struct FileUpload {
    pub data:Signal<Vec<(String, String, f64,i64)>>,
}


#[component]
pub fn BtnUplaod(file_upload:FileUpload) -> Element {
    let mut files_uploaded = file_upload.data;
    let mut pass_file = use_signal(|| String::new());
    let mut err_show = use_signal(|| String::new());

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            match read_credit_kbank(file_name, pass_file.read().as_ref()) {
                Ok(line) => {
                    files_uploaded.clear();
                    err_show.set(String::new());
                    let trafrom = line
                        .date
                        .iter()
                        .enumerate()
                        .map(|(i, _)| (line.date[i].clone(), line.ctx[i].clone(), line.amount[i] , line.label_id[i]))
                        .collect::<Vec<(String, String, f64,i64)>>();
                    files_uploaded.extend(trafrom);
                }
                Err(err) => {
                    println!("err1 {}", err);
                    println!("err2 {}", err_show.read().as_str());
                    err_show.set(format!("{}", err))
                }
            }
        }
    };
    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };
    rsx! {
        div { class: "flex items-center justify-center ",
            div {
                input {
                    r#type: "password",
                    class: "border border-2 border-black rounded-md mr-2 ml-2",
                    value: "{pass_file}",
                    oninput: move |event| pass_file.set(event.value()),
                }
            }
            div {
                label {
                    r#for:"upload-fantacy",
                    div {
                        class: "py-2 px-5 bg-violet-500 text-white font-semibold rounded-full shadow-md hover:bg-violet-700 focus:outline-none focus:ring focus:ring-violet-400 focus:ring-opacity-75",
                        {"Upload Files"}
                    }
                }

                    input {
                        class: "btnUpload",
                        id: "upload-fantacy",
                        r#type: "file",
                        style: "display:none;",
                        accept: ".pdf",
                        multiple: true,
                        name: "textreader",
                        onchange: upload_files,
                    }
                
                
            }
        }
    }
}

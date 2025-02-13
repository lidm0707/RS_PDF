use dioxus::prelude::*;
use dioxus_elements::FileEngine;
use std::sync::Arc;

use crate::backend::{
    controller::con_pdf::read_pdf::set_credit_kbank, model::model_pdf::TranformLine,
};

#[component]
pub fn BtnUplaod(file_upload: Signal<Vec<TranformLine>>) -> Element {
    let mut files_uploaded = file_upload;
    let mut pass_file: Signal<String> = use_signal(|| String::new());
    let mut err_show = use_signal(|| String::new());

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            match set_credit_kbank(file_name, pass_file.read().as_ref()) {
                Ok(line) => {
                    files_uploaded.clear();
                    err_show.set(String::new());
                    let trafrom = line
                        .date
                        .iter()
                        .enumerate()
                        .map(|(i, _)| TranformLine {
                            date: line.date[i].to_string(),
                            ctx: line.ctx[i].to_string(),
                            amount: line.amount[i],
                            label_id: line.label_id[i],
                            period: line.period[i].to_string(),
                            payment_type_id: line.payment_type_id[i],
                        })
                        .collect::<Vec<TranformLine>>();
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
                        class: "btn",
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

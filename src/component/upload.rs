use crate::service::pdf::read_pdf::read_credit_kbank;
use dioxus::prelude::*;
use dioxus_elements::FileEngine;
use std::sync::Arc;

#[component]
pub fn BtnUplaod() -> Element {
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<(String, String, f64)>);
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
                        .map(|(i, _)| (line.date[i].clone(), line.ctx[i].clone(), line.amount[i]))
                        .collect::<Vec<(String, String, f64)>>();
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
        div { class: "flex items-center justify-center",
            div {
                input {
                    r#type: "password",
                    // we tell the component what to render
                    value: "{pass_file}",
                    // and what to do when the value changes
                    oninput: move |event| pass_file.set(event.value()),
                }
            }
            div {
                button {
                    r#type: "button",
                    class: "btn bg-white",
                    id: "btn-upload",
                    popovertarget: "upload",
                    onclick: move |_| {
                        document::eval(&format!(r#"document.getElementById('input-upload').click();"#));
                    },
                    {"Upload Files"}
                }


                input {
                    class: "btnUpload",
                    id: "input-upload",
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
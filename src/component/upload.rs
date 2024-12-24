use std::sync::Arc;
use dioxus::prelude::*;
use dioxus_elements::FileEngine;
use crate::service::pdf::read_pdf::{read,Line};


#[component]
pub fn BtnUplaod () -> Element {
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<Line>);
    let mut pass_file = use_signal(|| String::new());

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            match read(file_name, pass_file.read().as_ref()) {
                Ok(line) => files_uploaded.push(line),
                Err(err) => eprintln!("Error reading file {}: {:?}", file_name, err),
            }
        }
    };
    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };
    rsx! {
        div { class: "flex flex-col items-center justify-center",
            div {
                input {
                    // we tell the component what to render
                    value: "{pass_file}",
                    // and what to do when the value changes
                    oninput: move |event| pass_file.set(event.value())
                }
            }
            div {
                label { r#for: "textreader", "Upload text/rust files and read them" }
                input {
                    r#type: "file",
                    accept: ".pdf",
                    multiple: true,
                    name: "textreader",
                    onchange: upload_files,
                }
            }

            div {
                {
                    files_uploaded
                        .read()
                        .iter()
                        .map(|line| {
                            rsx! {
                                p { "{line.date:?}" }
                            }
                        })
                }
            }
        }
    }
    


}


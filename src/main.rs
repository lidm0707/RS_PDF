use diesel::SqliteConnection;
use dioxus::desktop::{tao, LogicalPosition};
use dioxus::prelude::*;
use rust_pdf::component::menu::MenuButton;
use rust_pdf::database::{
    db_connect::connect_database,
    db_select::{select_labels, select_raws},
};
use rust_pdf::page::pageRaw::{PageRaw,TableRaw};
use rust_pdf::page::pageLabel::{PageLabel,Tablelabel};
use rust_pdf::page::pageUpload::{PageUpload, TableUpload};

fn main() {
    let window = tao::window::WindowBuilder::new()
        .with_resizable(true)
        .with_position(LogicalPosition::new(0.0, 0.0));
    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                // .with_disable_context_menu(true)
                .with_window(window),
        )
        .launch(App);
    // dioxus::launch(App);
}

pub enum Content {
    Label,
    Raw,
    Upload,
}

#[component]
fn App() -> Element {
    let mut conn: SqliteConnection = connect_database();
    let table_label: Signal<Vec<rust_pdf::models::model_label::SelectLabels>> = use_signal(|| select_labels(&mut conn).unwrap());
    let table_raw: Signal<Vec<rust_pdf::models::model_raw::SelectRaws>> = use_signal(|| select_raws(&mut conn).unwrap());
    let mut show_labels = use_signal(|| Some(Content::Label));
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<(String, String, f64)>);
    rsx! {
      document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

      div { class: "main",
        div { class: "menu",
          div {
            h5 { "Sidebar" }
          }
          MenuButton {
            onclick: move |_| {
                show_labels.set(Some(Content::Label));
            },
          }
          MenuButton {
            onclick: move |_| {
                show_labels.set(Some(Content::Raw));
            },
          }
          MenuButton {
            onclick: move |_| {
                show_labels.set(Some(Content::Upload));
            },
          }
        }

        match show_labels.read().as_ref().unwrap() {
            Content::Label => {
                rsx! {
                  PageLabel { data: table_label }
                }
            }
            Content::Raw => {
                rsx! {
                  PageRaw { data: table_raw }
                }
            }
            Content::Upload => {
                rsx! {
                  PageUpload{data:files_uploaded}
                }
            }
        }
      }
    }
}

use dioxus::desktop::{tao, LogicalPosition};
use dioxus::prelude::*;
use rust_pdf::component::menu::MenuButton;
use rust_pdf::page::page_credit::content_credit;
use rust_pdf::page::page_label::content_label;
use rust_pdf::page::page_installment::content_installment;
use rust_pdf::page::page_upload::content_upload;

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
    INSTALLMENT,
}

#[component]
fn App() -> Element {
    let mut show_content = use_signal(|| Content::Label);
    rsx! {
      document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

      div { class: "main",
        div { class: "menu",
          div {
            h5 { "Menu" }
          }
          MenuButton {
            onclick: move |_| {
                show_content.set(Content::Label);
            },
            name: "LABEL",
          }
          MenuButton {
            onclick: move |_| {
                show_content.set(Content::Raw);
            },
            name: "CREKBANK",
          }
          MenuButton {
            onclick: move |_| {
                show_content.set(Content::INSTALLMENT);
            },
            name: "INSTALLMENT",
          }
          MenuButton {
            onclick: move |_| {
                show_content.set(Content::Upload);
            },
            name: "UPLOAD",
          }
        }

        match *show_content.read() {
            Content::Label => {
                rsx! {
                  content_label {}
                }
            }
            Content::Raw => {
                rsx! {
                  content_credit {}
                }
            }
            Content::Upload => {
                rsx! {
                  content_upload {}
                }
            }
            Content::INSTALLMENT =>{
              rsx! {
                content_installment{}
              }
            }
        }
      }
    }
}

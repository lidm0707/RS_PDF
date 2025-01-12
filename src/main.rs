use dioxus::desktop::{tao, LogicalPosition};
use dioxus::prelude::*;
use rust_pdf::component::menu::MenuButton;
use rust_pdf::page::page_cash::content_cash;
use rust_pdf::page::page_credit::content_credit;
use rust_pdf::page::page_dashboad::content_dashboard_credit;
use rust_pdf::page::page_installment::content_installment;
use rust_pdf::page::page_label::content_label;
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
        .launch(app);
    // dioxus::launch(App);
}

pub enum Content {
    Label,
    Raw,
    Upload,
    Installment,
    Cash,
    Dashboard,
}

#[component]
fn app() -> Element {
    let mut show_content = use_signal(|| Content::Dashboard);
    rsx! {
      document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
      div { class: "main",
        div { class: "menu",
          div {
            h5 { "Menu" }
          }
          MenuButton {
            onclick: move |_| {
                show_content.set(Content::Dashboard);
            },
            name: "Dashboard",
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
                show_content.set(Content::Installment);
            },
            name: "INSTALLMENT",
          }
          MenuButton {
            onclick: move |_| {
                show_content.set(Content::Upload);
            },
            name: "UPLOAD",
          }
          MenuButton {
            onclick: move |_| {
                show_content.set(Content::Cash);
            },
            name: "CASH",
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
            Content::Installment =>{
              rsx! {
                content_installment{}
              }
            }
            Content::Cash => {
              rsx! {
                content_cash{}
              }
            }
            Content::Dashboard => {
              rsx! {
                content_dashboard_credit{}
              }
            }
        }
      }
    }
}

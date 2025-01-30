use dioxus::desktop::{tao, LogicalPosition};
use dioxus::prelude::*;
use rust_pdf::component::menu::MenuButton;
use rust_pdf::page::{
    page_cash, page_credit, page_dashboad, page_installment, page_label, page_upload,
};
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
    // dioxus::launch(app);
}

pub enum Content {
    Label,
    Raw,
    Upload,
    Installment,
    Cash,
    Dashboard,
}

fn app() -> Element {
    let mut show_content = use_signal(|| Content::Dashboard);

    rsx! {
      document::Link { rel: "stylesheet", href: asset!("/assets/output.css") }
      div { class: "main",
        div { class: "menu",
          div {
            h5 { "Menu V.1.0.0" }
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
                  page_label::content_label{}
                }
            }
            Content::Raw => {
                rsx! {
                  page_credit::content_credit{}
                }
            }
            Content::Upload => {
                rsx! {
                  page_upload::content_upload {}
                }
            }
            Content::Installment => {
                rsx! {
                  page_installment::content_installment {}
                }
            }
            Content::Cash => {
                rsx! {
                  page_cash::content_cash {}
                }
            }
            Content::Dashboard => {
                rsx! {
                  page_dashboad::content_dashboard{}
                }
            }
        }
      }
    }
}

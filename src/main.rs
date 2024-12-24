use diesel::SqliteConnection;
use dioxus::desktop::{tao, LogicalPosition};
use dioxus::prelude::*;
use rust_pdf::component::datepicker::Picker;
use rust_pdf::component::menu::MenuButton;
use rust_pdf::{
    component::table::{Label, Raw},
    database::{
        db_connect::connect_database,
        db_select::{select_labels, select_raws},
    },
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
        .launch(App);
    // dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut conn: SqliteConnection = connect_database();
    let table_label = use_signal(|| select_labels(&mut conn).unwrap());
    let table_raw = use_signal(|| select_raws(&mut conn).unwrap());
    let mut show_labels = use_signal(|| true);

    rsx! {
      document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

      div { class: "main",
        div { class: "menu",
          div {
            h5 { "Sidebar" }
          }
          MenuButton {
            onclick: move |_| {
                let current_value = *show_labels.read();
                show_labels.set(!current_value);
            },
          }
        }
        div { class: "content",
          div { class: "summary" ,}
          div { class: "control" , Picker{} }
          
          
          if *show_labels.read() {
            Label { table: table_label }
          } else {
            Raw { table: table_raw }
          }
        }
      }
    }
}

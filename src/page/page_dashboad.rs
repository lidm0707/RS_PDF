use std::collections::HashMap;

use chrono::prelude::*;
use dioxus::prelude::*;

use crate::{
    backend::controller::{
        con_dashboard::{
             con_dash_cash_credit::get_dashboard_cash_credit, con_dash_net::get_dashboard_net
        },
        con_date_handle::con_now::get_thai_now,
    },
    component::{
        com_select::select_custome::select_onchang,
        com_table::{
            table_cash_credit_dashboard::CashCreditDashboardTable, table_net_dashboard::NetDashboardTable
        },
    },
};

pub fn content_dashboard() -> Element {
    let  month = use_signal(|| get_thai_now().month().to_string()); // Default to January
    let  year = use_signal(|| get_thai_now().year().to_string()); // Default to 2025

    let mut start = use_signal(|| format!("{}-{}", year, month).to_string());
    let mut end = use_signal(|| format!("{}-12", year).to_string());


    let mut data_table_net: Signal<Vec<(String, f64, f64)>> =
        use_signal(|| get_dashboard_net(&start.read(), &end.read()).unwrap());
        let mut data_table_cash_credit: Signal<Vec<(String, HashMap<i32, HashMap<String, f64>>)>> =
        use_signal(|| get_dashboard_cash_credit(&start.read(), &end.read()));

    use_effect(move || {
        let year_val = year.read();
        let month_val = month.read();

        // Format the start and end period based on selected month and year
        let new_start = format!("{year_val}-{month_val:0>2}");
        let new_end = format!("{year_val}-12");

        start.set(new_start);
        end.set(new_end);
        data_table_cash_credit.set(get_dashboard_cash_credit(&start.read(), &end.read()));
        data_table_net.set(get_dashboard_net(&start.read(), &end.read()).unwrap());
    });

    let now_thai = get_thai_now();
    let arr_year: Vec<i32> = (now_thai.year() - 5..=now_thai.year() + 5).collect();

    rsx! {
        div { class: "content",
            div { class: "control",

                select_onchang {
                    data: month.clone(),
                    option: {
                        rsx! {
                            {
                                (1..=12)
                                    .map(|m| {
                                        let temp_month = month.read();
                                        let imonth = temp_month.parse::<u32>().unwrap_or(1u32);
                                        rsx! {
                                            option { value: "{m}", selected: "{m == imonth}", "{m}" }
                                        }
                                    })
                            }
                        }
                    },
                }

                select_onchang {
                    data: year.clone(),
                    option: {
                        rsx! {
                            {
                                arr_year
                                    .iter()
                                    .map(|y| {
                                        let temp_year = year.read();
                                        let iyear = temp_year.parse::<i32>().unwrap_or(2025);
                                        rsx! {
                                            option { value: "{y}", selected: "{*y == iyear}", "{y}" }
                                        }
                                    })
                            }
                        }
                    },
                }
            }
            p { "test" }
            CashCreditDashboardTable { data_table: data_table_cash_credit }
            p { "NET" }
            NetDashboardTable { data_table: data_table_net }

        }
    }
}

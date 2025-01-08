use std::collections::HashMap;

use chrono::prelude::*;
use dioxus::prelude::*;

use crate::{
    component::{
        com_date::datepicker::PickerDate,
        com_select::select_custome::select_onchang,
        com_table::{
            table_cash_dashboard::CashDashboardTable, table_credit_dashboard::CreditDashboardTable,
        },
    },
    controller::con_dashboard::{
        con_dash_cash::get_dashboard_cash, con_dash_credit::get_dashboard_credit,
    },
    entity::entity_credit::SelectCredit,
    service::date::now::thai_now,
};

#[derive(PartialEq, Clone, Props)]
pub struct TableRaw {
    pub data: Signal<Vec<SelectCredit>>,
}

pub fn content_dashboard_credit() -> Element {
    let mut month = use_signal(|| "1".to_string()); // Default to January
    let mut year = use_signal(|| "2025".to_string()); // Default to 2025

    let mut start = use_signal(|| "2025-01".to_string());
    let mut end = use_signal(|| "2025-12".to_string());

    let mut data_table: Signal<Vec<(String, Vec<Option<f64>>)>> =
        use_signal(|| get_dashboard_credit(&start.read(), &end.read()).unwrap());

    let mut data_table_cash: Signal<Vec<(String, Vec<Option<f64>>)>> =
        use_signal(|| get_dashboard_cash(&start.read(), &end.read()).unwrap());

    // Update `start` and `end` whenever `year` or `month` changes

    use_effect(move || {
        let year_val = year.read();
        let month_val = month.read();

        // Format the start and end period based on selected month and year
        let new_start = format!("{year_val}-{month_val:0>2}");
        let new_end = format!("{year_val}-12");

        start.set(new_start);
        end.set(new_end);

        data_table.set(get_dashboard_credit(&start.read(), &end.read()).unwrap());
        data_table_cash.set(get_dashboard_cash(&start.read(), &end.read()).unwrap());
    });

    let now_thai = thai_now();
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

            CreditDashboardTable { data_table }
            CashDashboardTable { data_table: data_table_cash }
        }
    }
}

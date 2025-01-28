use dioxus::prelude::*;

use crate::{backend::{
    controller::con_db::{con_get_bank::get_bank_where, con_get_installment::get_installment},
    model::model_installment::{ModelInstallment, ModelInstallmentItems},
}, format::format_with_separator};

#[component]
pub fn TableInstallmentItem(
    df_installment_items: Signal<Vec<ModelInstallmentItems>>,
    df_installment: Signal<Vec<ModelInstallment>>,
    id_table: Signal<i32>,
) -> Element {
    // pub id: i32,
    // pub date: String,
    // pub period: String,
    // pub bank_id:i32,
    // pub amount:f64,
    // pub installment_id: i32,
    rsx! {
        table {
            thead {
                tr {
                    th { "DATE" }
                    th { "PERIOD" }
                    th { "BANK_ID" }
                    th { "AMOUNT" }
                }
            }
            tbody {
                {
                    &mut df_installment_items
                        .iter()
                        .map(|installment| {
                            rsx! {
                                tr {
                                    td {
                                        onclick: move |_| {
                                            id_table.set(0i32);
                                            id_table.read();
                                            println!("{}", id_table.read());
                                            df_installment.set(get_installment().expect("Failed to load labels"));
                                        },
                                        "{installment.date}"
                                    }
                                    td { "{installment.period}" }
                                    td {
                                        {
                                            let bank_id = installment.bank_id;
                                            get_bank_where(bank_id).unwrap()[0].name.clone()
                                        }
                                    }
                                    td { "{format_with_separator(&installment.amount)}" }
                                }
                            }
                        })
                }
            }
        }
    }
}

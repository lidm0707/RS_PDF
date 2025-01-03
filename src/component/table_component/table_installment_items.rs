
use dioxus::prelude::*;

use crate::{database::{db_bank::db_select::select_labels_where, db_installment::db_select::select_installment}, entity::entity_installment::{SelectInstallment, SelectInstallmentItems}};

#[component]
pub fn TableInstallmentItem(df_installment_items:Signal<Vec<SelectInstallmentItems>>,df_installment:Signal<Vec<SelectInstallment>>, id_table:Signal<i32>) -> Element {

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
                                            df_installment.set(select_installment().expect("Failed to load labels"));
                                        },
                                        "{installment.date}"
                                    }
                                    td { "{installment.period}" }
                                    td {
                                        {
                                            let bank_id = installment.bank_id;
                                            select_labels_where(bank_id).unwrap()[0].name.clone()
                                            
                                        }
                                    }
                                    td { "{installment.amount}" }
                                }
                            }
                        })
                }
            }
        }
    }
}
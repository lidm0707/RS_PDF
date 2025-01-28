use dioxus::prelude::*;

use crate::{backend::{
        controller::con_db::{
            con_get_installment::get_installment_items_where, con_get_label::get_label_name_where,
        },
        model::model_installment::{ModelInstallment, ModelInstallmentItems},
    }, format::format_with_separator};
// /    let mut df_installment_items: Signal<Vec<SelectInstallmentItems>> = use_signal(|| select_installment_items_where(*id_table.read()).expect("Failed to load labels"));

#[component]
pub fn TableInstallment(
    df_installment: Signal<Vec<ModelInstallment>>,
    df_installment_items: Signal<Vec<ModelInstallmentItems>>,
    id_table: Signal<i32>,
) -> Element {
    rsx! {
        table {
            thead {
                tr {
                    th { "DATE_START" }
                    th { "DATE_END" }
                    th { "TIME" }
                    th { "NOTE" }
                    th { "LABEL_ID" }
                    th { "AMOUNT" }
                    th { "TOTAL" }
                }
            }
            tbody {
                {
                    &mut df_installment
                        .iter()
                        .map(move |installment| {
                            let ins_id = installment.id.clone() as i32;
                            rsx! {
                                tr {
                                    td {
                                        onclick: move |_| {
                                            println!("{}", ins_id);
                                            id_table.set(ins_id);
                                            df_installment_items
                                                .set(get_installment_items_where(ins_id).expect("Failed to load labels"));
                                        },
                                        "{installment.date_start}"
                                    }
                                    td { "{installment.date_end}" }
                                    td { "{installment.time}" }
                                    td { "{installment.note}" }
                                    td {
                                        {
                                            let input_id = installment.label_id.clone() as i32;
                                            get_label_name_where(input_id).unwrap()[0].label.clone()
                                        }
                                    }
                                    td { "{format_with_separator(&installment.amount)}" }
                                    td { "{format_with_separator(&installment.total)}" }
                                }
                            }
                        })
                }
            }
        }
    }
}

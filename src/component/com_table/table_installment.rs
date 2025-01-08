use dioxus::prelude::*;

use crate::{
    repo::{
        db_installment::db_select::select_installment_items_where,
        db_label::db_select::select_labels_name_where,
    },
    entity::entity_installment::{SelectInstallment, SelectInstallmentItems},
};
// /    let mut df_installment_items: Signal<Vec<SelectInstallmentItems>> = use_signal(|| select_installment_items_where(*id_table.read()).expect("Failed to load labels"));

#[component]
pub fn TableInstallment(
    df_installment: Signal<Vec<SelectInstallment>>,
    df_installment_items: Signal<Vec<SelectInstallmentItems>>,
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
                                                .set(select_installment_items_where(ins_id).expect("Failed to load labels"));
                                        },
                                        "{installment.date_stard}"
                                    }
                                    td { "{installment.date_end}" }
                                    td { "{installment.time}" }
                                    td { "{installment.note}" }
                                    td {
                                        {
                                            let input_id = installment.label_id.clone() as i32;
                                            select_labels_name_where(input_id).unwrap()[0].label.clone()
                                        }
                                    }
                                    td { "{installment.amount}" }
                                    td { "{installment.total}" }
                                }
                            }
                        })
                }
            }
        }
    }
}

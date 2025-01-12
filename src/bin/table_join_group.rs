use rust_pdf::repo::db_dashboard::db_select::union_installment_credit;


fn main(){
    let _ = println!("{:?}",union_installment_credit("2025-01","2025-03"));
}
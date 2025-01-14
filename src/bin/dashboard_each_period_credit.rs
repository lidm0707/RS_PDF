use rust_pdf::backend::service::tranformdata::dashboard_credit::table_period_credit;

fn main(){
    for i in  table_period_credit("2025-01","2025-04").unwrap().iter() {
        println!("{:?}",i);
    }
    
}
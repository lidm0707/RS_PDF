use std::cmp::Ordering;

use rust_pdf::database::db_credit::db_select::select_credit_groupby_label;



fn main(){
    let mut data = select_credit_groupby_label("2025-01","2025-02").unwrap();
    


    data.sort_by(|a, b| {
        let period_cmp = b.period.cmp(&a.period);
        if period_cmp == Ordering::Equal {
            b.amount
                .unwrap_or(0.0) // ใช้ค่าเริ่มต้นหากเป็น None
                .partial_cmp(&a.amount.unwrap_or(0.0)) // ใช้ partial_cmp เพื่อเปรียบเทียบค่าแบบ f64
                .unwrap_or(Ordering::Equal) // หากค่าไม่สามารถเปรียบเทียบได้ ให้ถือว่าเท่ากัน
        } else {
            period_cmp
        }
    });

    // แสดงผลลัพธ์
    for item in &data {
        println!("{:?}", item);
    }
}
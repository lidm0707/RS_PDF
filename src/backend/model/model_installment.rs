#[derive(Debug,Clone)]
pub struct ModelInstallment {
    pub id: i32,
    pub date_start: String,
    pub date_end: String,
    pub time: i32,
    pub note: String,
    pub label_id: i32,
    pub amount: f64,
    pub total: f64,
}


#[derive(Debug,Clone)]

pub struct ModelInstallmentItems {
    pub id: i32,
    pub date: String,
    pub period: String,
    pub bank_id:i32,
    pub amount:f64,
    pub installment_id: i32,
}
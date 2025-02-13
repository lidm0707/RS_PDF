
#[derive(Debug,Clone)]
pub struct Line {
    pub date: Vec<String>,
    pub ctx: Vec<String>,
    pub amount: Vec<f64>,
    pub label_id:Vec<i64>,
    pub period:Vec<String>,
    pub payment_type_id:Vec<i64>,
}
#[derive(Debug,Clone)]
pub struct TranformLine{
    pub date: String,
    pub ctx: String,
    pub amount: f64,
    pub label_id:i64,
    pub period:String,
    pub payment_type_id:i64,
}
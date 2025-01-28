use crate::backend::service::sv_data::sv_get_plan::{sv_get_plan_amount_where, sv_get_plan_by_period};


pub fn get_plan_where(input_period: &str, input_label_id: i32) -> f64 {
    // Safely call `select_payment_type_where` and propagate errors
    let result = sv_get_plan_amount_where(input_period, input_label_id);
    result
}



pub fn get_plan_by_period(input_period: &str) -> (String, f64)  {
    // Safely call `select_payment_type_where` and propagate errors
    let result = sv_get_plan_by_period(input_period);
    result
}
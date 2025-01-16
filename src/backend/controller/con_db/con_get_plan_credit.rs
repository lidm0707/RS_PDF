use crate::backend::service::sv_data::sv_get_plan_credit::sv_get_plan_credit_amount_where;


pub fn get_plan_credit_where(input_period: &str, input_label_id: i32) -> f64 {
    // Safely call `select_payment_type_where` and propagate errors
    let result = sv_get_plan_credit_amount_where(input_period, input_label_id);
    result
}

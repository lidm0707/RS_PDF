use crate::backend::repo::db_plan_credit::db_select::select_plan_credit_where;

pub fn sv_get_plan_credit_amount_where(input_period: &str, input_label_id: i32) -> f64 {
    // Safely call `select_payment_type_where` and propagate errors
    let result = select_plan_credit_where(input_period, input_label_id)
        .unwrap()
        .last() // Get the first element in the vector
        .map(|credit| credit.amount) // Map to `amount` from the `ModelPlanCredit`
        .unwrap_or(0.0); // If there's no credit, return 0.;
    result
}

use crate::backend::repo::db_plan::db_select::{select_plan_by_period, select_plan_where};

pub fn sv_get_plan_amount_where(input_period: &str, input_label_id: i32) -> f64 {
    // Safely call `select_payment_type_where` and propagate errors
    let result = select_plan_where(input_period, input_label_id)
        .unwrap()
        .last() // Get the first element in the vector
        .map(|credit| credit.amount) // Map to `amount` from the `ModelPlanCredit`
        .unwrap_or(0.0); // If there's no credit, return 0.;
    result
}

pub fn sv_get_plan_by_period(input_period: &str) -> (String, f64) {
    // Safely call `select_plan_by_period` and handle the result
    match select_plan_by_period(input_period) {
        Ok(result) => result, // Return the result if successful
        Err(e) => {
            // Handle the error case
            eprintln!("Error fetching plan by period: {}", e); // Log the error
            (String::from("Unknown"), 0.0) // Return a default value
        }
    }
}

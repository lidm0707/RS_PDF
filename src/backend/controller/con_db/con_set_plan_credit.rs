use crate::backend::{
    model::model_plan_credit::ModelPlanCredit,
    service::sv_data::sv_set_plan_credit::sv_set_plan_credit,
};

pub fn set_plan_credit(period: String, label_id: i32, amount: f64) -> ModelPlanCredit {
    // Safely call `select_payment_type_where` and propagate errors
    let result = sv_set_plan_credit(period, label_id, amount);

    result
}

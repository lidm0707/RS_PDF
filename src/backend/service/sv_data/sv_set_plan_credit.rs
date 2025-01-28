use crate::backend::{
    model::model_plan_credit::ModelPlanCredit, repo::db_plan::db_insert::insert_plan_credit,
};

pub fn sv_set_plan_credit(period: String, label_id: i32, amount: f64) -> ModelPlanCredit {
    let raw_data = insert_plan_credit(period, label_id, amount);
    let result = ModelPlanCredit {
        id: raw_data.id,
        period: raw_data.period,
        label_id: raw_data.label_id,
        amount: raw_data.amount,
    };

    result
}

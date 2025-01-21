use crate::backend::{
    model::model_revenue_type::ModelRevenueType,
    service::sv_data::sv_get_revenue_type::{sv_get_revenue_type, sv_get_revenue_type_where},
};

pub fn con_get_revenue_type() -> Result<Vec<ModelRevenueType>,anyhow::Error> {
    let result = sv_get_revenue_type().unwrap();
    Ok(result)
}


pub fn con_get_revenue_type_where(input_id: i32) -> Result<Vec<ModelRevenueType>,anyhow::Error> {
    let result = sv_get_revenue_type_where(input_id).unwrap();
    Ok(result)
}


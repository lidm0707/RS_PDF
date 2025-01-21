use crate::backend::{
    model::model_revenue_type::ModelRevenueType,
    service::sv_data::sv_set_revenue_type::sv_set_revenue_type,
};

pub fn con_set_revenue_type(category: &str) -> Result<ModelRevenueType,anyhow::Error> {
    let result = sv_set_revenue_type(category).unwrap();
    Ok(result)
}

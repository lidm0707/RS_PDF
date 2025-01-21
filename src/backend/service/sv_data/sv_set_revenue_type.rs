use crate::backend::{
    model::model_revenue_type::ModelRevenueType,
    repo::db_revenue_type::db_insert::insert_revenue_type,
};

pub fn sv_set_revenue_type(category: &str) -> Result<ModelRevenueType, anyhow::Error> {
    let raw = insert_revenue_type(category.to_string());
    let result = ModelRevenueType {
        id: raw.id,
        category: raw.category,
    };

    Ok(result)
}

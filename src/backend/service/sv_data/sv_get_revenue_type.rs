use crate::backend::{model::model_revenue_type::ModelRevenueType, repo::db_revenue_type::db_select::{select_revenue_type, select_revenue_type_where}};



pub fn sv_get_revenue_type() -> Result<Vec<ModelRevenueType>, anyhow::Error> {
    let raw_data = select_revenue_type().unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelRevenueType {
            id: raw.id,
            category:raw.category,
        })
        .collect();

    Ok(result)
}


pub fn sv_get_revenue_type_where(input_id:i32) -> Result<Vec<ModelRevenueType>, anyhow::Error> {
    let raw_data = select_revenue_type_where(input_id).unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelRevenueType {
            id: raw.id,
            category:raw.category,
        })
        .collect();

    Ok(result)
}



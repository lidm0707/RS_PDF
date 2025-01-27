use crate::backend::{
    model::model_cash_in::ModelCashIn, 
    repo::db_cash_in::db_select::select_cash_in
};

pub fn sv_get_cash_in() -> Result<Vec<ModelCashIn>, anyhow::Error> {
    // Fetch raw data from the database
    let raw_data = select_cash_in()?; // Assuming `select_cash` returns `Result<Vec<SelectCash>, anyhow::Error>`
    
    // Transform `SelectCash` into `ModelCash`
    let result = raw_data
        .into_iter()
        .map(|raw| ModelCashIn {
            id: raw.id,
            date: raw.date,
            period: raw.period,
            revenue_id: raw.revenue_id,
            note:raw.note,
            amount: raw.amount,
        })
        .collect();
    
    Ok(result)
}

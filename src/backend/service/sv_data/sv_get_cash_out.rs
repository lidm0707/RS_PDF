use crate::backend::{
    model::model_cash_out::ModelCashOut, 
    repo::db_cash_out::db_select::select_cash_out
};

pub fn sv_get_cash_out() -> Result<Vec<ModelCashOut>, anyhow::Error> {
    // Fetch raw data from the database
    let raw_data = select_cash_out()?; // Assuming `select_cash` returns `Result<Vec<SelectCash>, anyhow::Error>`
    
    // Transform `SelectCash` into `ModelCash`
    let result = raw_data
        .into_iter()
        .map(|raw| ModelCashOut {
            id: raw.id,
            date: raw.date,
            period: raw.period,
            label_id: raw.label_id,
            note:raw.note,
            amount: raw.amount,
        })
        .collect();
    
    Ok(result)
}

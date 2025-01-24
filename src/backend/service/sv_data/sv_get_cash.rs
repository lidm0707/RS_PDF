use crate::backend::{
    model::model_cash::ModelCash, 
    repo::db_cash::db_select::select_cash
};

pub fn sv_get_cash() -> Result<Vec<ModelCash>, anyhow::Error> {
    // Fetch raw data from the database
    let raw_data = select_cash()?; // Assuming `select_cash` returns `Result<Vec<SelectCash>, anyhow::Error>`
    
    // Transform `SelectCash` into `ModelCash`
    let result = raw_data
        .into_iter()
        .map(|raw| ModelCash {
            id: raw.id,
            date: raw.date,
            period: raw.period,
            type_cash: raw.type_cash,
            label_id: raw.label_id,
            note:raw.note,
            amount: raw.amount,
        })
        .collect();
    
    Ok(result)
}

use crate::backend::{
    model::model_bank::ModelBank,
    repo::db_bank::db_select::{select_bank, select_bank_where},
};

pub fn get_bank() -> Result<Vec<ModelBank>, anyhow::Error> {
    let raw_data = select_bank().unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelBank {
            id: raw.id,
            name: raw.name,
        })
        .collect();

    Ok(result)
}

pub fn get_bank_where(input: i32) -> Result<Vec<ModelBank>, anyhow::Error> {
    let raw_data = select_bank_where(input).unwrap();
    let result = raw_data
        .into_iter()
        .map(|raw| ModelBank {
            id: raw.id,
            name: raw.name,
        })
        .collect();

    Ok(result)
}

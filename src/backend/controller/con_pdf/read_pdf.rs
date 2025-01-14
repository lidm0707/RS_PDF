use crate::backend::{model::model_pdf::Line, service::pdf::read_pdf::read_credit_kbank};

pub fn set_credit_kbank(file_path: &str, password: &str) -> Result<Line, anyhow::Error> {
    let result = read_credit_kbank(file_path, password).unwrap();
    Ok(result)
}

use crate::backend::{model::model_pdf::Line, service::pdf::read_pdf::read_credit_kbank};

pub fn set_credit_kbank(file_path: &str, password: &str) -> Result<Line, anyhow::Error> {
    match read_credit_kbank(file_path, password) {
        Ok(value) => Ok(value),
        Err(err) => {
            Err(err)
        }
    }
}
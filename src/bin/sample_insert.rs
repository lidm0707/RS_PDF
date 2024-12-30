use anyhow;
use database::{db_connect::connect_database, db_insert::{insert_credit, insert_label}, db_select::select_labels};
use dotenv::dotenv;
use rust_pdf::*;
use service::pdf::read_pdf;
use std::env;

fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let password = env::var("password").unwrap();
    let file_path = env::var("PDFNAME").unwrap();
    let mut conn = connect_database();
    let data = read_pdf::read_credit_kbank(&file_path, &password)?;
    let mut total: f64 = 0.0;
    for (index, item) in data.amount.iter().enumerate() {
        let _ = insert_credit(
            &mut conn,
            data.date[index].clone(),
            data.ctx[index].clone(),
            *item,
            "label".to_string(),
        );
        total += item;
    }
    println!("total:// {total:.2}");

    let _ = select_labels();
    // create_post_raw

    Ok(())
}

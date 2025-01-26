use crate::{
    backend::model::model_pdf::Line,
    backend::service::{
        date::{date_format::format_date, now::thai_now},
        pdf::check_label::search_labels,
    },
};
use chrono::prelude::*;
use pdfium_render::prelude::*;
use regex::Regex;

pub fn read_credit_kbank(file_path: &str, password: &str) -> Result<Line, anyhow::Error> {
    let mut data = Line {
        date: Vec::new(),
        ctx: Vec::new(),
        amount: Vec::new(),
        label_id: Vec::new(),
        period: Vec::new(),
        payment_type_id: Vec::new(),
    };

    let date_regex = Regex::new(r"^\d{2}/\d{2}/\d{2}").unwrap();
    let check_password = if password.is_empty() {
        None
    } else {
        Some(password)
    };

    match Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./assets")) {
        Ok(bind) => {
            let pdf_raw = Pdfium::new(bind);
            let pdf = pdf_raw.load_pdf_from_file(file_path, check_password)?;
            let pages = pdf.pages();
            let total_pages = pages.len();
            println!("Total pages: {}", total_pages);

            for (index, page) in pages.iter().enumerate() {
                if let Ok(text) = page.text() {
                    let content = text.all(); // Get all text from the page
                    let lines: Vec<&str> = content.lines().collect(); // Split content into lines

                    for line in &lines {
                        // Print lines matching date format
                        if date_regex.is_match(line) {
                            split_line(line, total_pages as u16, index as u16, &mut data)?;
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("{:?}", err)
        }
    };

    // Print debug information for the collected data
    println!("{:?}", data);
    Ok(data)
}

fn split_line(
    line: &str,
    total_pages: u16,
    index: u16,
    data: &mut Line,
) -> Result<(), anyhow::Error> {
    let arr: Vec<&str> = line.trim().split_whitespace().collect();

    // Log line content for debugging
    println!("Processing line: {}", line);

    // Ensure we have enough data in the line
    if arr.len() <= 2 || index + 1 >= total_pages {
        println!("Skipping line: {} - insufficient data", line);
    } else {
        // Extract components
        let date = arr[1].to_string();
        let ctx = arr[2..arr.len() - 1].join(" ");
        let amount_str = arr.last().unwrap();
        let (label_search_id, payment_type_id) = search_labels(&ctx).unwrap().unwrap(); // todo
                                                                                        ////////////////////////////////////////////////////////////////////////////////////
                                                                                        // Remove commas from the amount string
        let sanitized_amount_str = amount_str.replace(",", "");
        let now_thai = thai_now();
        // Attempt to parse the amount as f64
        match sanitized_amount_str.parse::<f64>() {
            Ok(amount) => {
                if amount >= 0.0 {
                    data.date.push(format_date(&date));
                    data.ctx.push(ctx);
                    data.amount.push(amount);
                    data.label_id.push(label_search_id as i64);
                    data.period
                        .push(format!("{}-{:02}", now_thai.year(), now_thai.month()));
                    data.payment_type_id.push(payment_type_id as i64);
                }
            }
            Err(e) => {
                // Handle the parsing error (e.g., log it or return an error)
                println!(
                    "Failed to parse amount '{}' on line '{}': {}",
                    amount_str, line, e
                );
                return Err(anyhow::anyhow!("Failed to parse amount: {}", amount_str));
            }
        }
    }

    Ok(())
}

use anyhow::Result;
use pdfium_render::prelude::*;
use regex::Regex;

use crate::service::pdf::check_label::search_labels;

#[derive(Debug)]
pub struct Line {
    pub date: Vec<String>,
    pub ctx: Vec<String>,
    pub amount: Vec<f64>,
    pub label_id:Vec<i64>,
}

pub fn read_credit_kbank(file_path: &str, password: &str) -> Result<Line> {
    let mut data = Line {
        date: Vec::new(),
        ctx: Vec::new(),
        amount: Vec::new(),
        label_id: Vec::new(),
    };
    
    let date_regex = Regex::new(r"^\d{2}/\d{2}/\d{2}").unwrap();
    let pdfium = Pdfium::default();

    let check_password = if password.is_empty() {
        None
    } else {
        Some(password)
    };
    let pdf = pdfium.load_pdf_from_file(file_path, check_password)?;

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

    // Print debug information for the collected data
    println!("{:?}", data);
    Ok(data)
}

fn split_line(line: &str, total_pages: u16, index: u16, data: &mut Line) -> Result<()> {
    let arr: Vec<&str> = line.trim().split_whitespace().collect();

    // Log line content for debugging
    println!("Processing line: {}", line);

    // Ensure we have enough data in the line
    if arr.len() <= 2 || index + 1 >= total_pages {
        println!("Skipping line: {} - insufficient data", line);
    } else {
        // Extract components
        let date = arr[1].to_string(); // Assuming the first word is the date
        let ctx = arr[2..arr.len() - 1].join(" "); // Everything except the first and last word is the context
        let amount_str = arr.last().unwrap(); // Last word is the amount
        let label_search_id = search_labels(&ctx).unwrap().unwrap() as i64;

        // Remove commas from the amount string
        let sanitized_amount_str = amount_str.replace(",", "");

        // Attempt to parse the amount as f64
        match sanitized_amount_str.parse::<f64>() {
            Ok(amount) => {
                if amount >= 0.0 {
                    data.date.push(date);
                    data.ctx.push(ctx);
                    data.amount.push(amount);
                    data.label_id.push(label_search_id);
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

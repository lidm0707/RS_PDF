use std::collections::HashMap;
use anyhow::Result;
// use rust_pdf::backend::repo::db_cash::db_select::select_cash_groupby_label;
// use rust_pdf::backend::repo::db_credit::db_select::select_credit_groupby_label;

// Mock the data structure used in the main function
#[derive(Clone)]
struct MockEntry {
    pub period: String,
    pub label_id: i32,
    pub amount: Option<f64>,
}

fn mock_select_cash_groupby_label(start: &str, end: &str) -> Result<Vec<MockEntry>, anyhow::Error> {
    Ok(vec![
        MockEntry { period: "2025-01".to_string(), label_id: 1, amount: Some(100.0) },
        MockEntry { period: "2025-01".to_string(), label_id: 2, amount: Some(200.0) },
    ])
}

fn mock_select_credit_groupby_label(start: &str, end: &str) -> Result<Vec<MockEntry>, anyhow::Error> {
    Ok(vec![
        MockEntry { period: "2025-01".to_string(), label_id: 0, amount: Some(150.0) },
    ])
}

// Function to summarize cash and credit data (from main code)
pub fn data_sumary_cash(
    start: &str,
    end: &str,
) -> Result<HashMap<String, HashMap<String, HashMap<i32, f64>>>, anyhow::Error> {
    let data_cash = mock_select_cash_groupby_label(start, end)
        .map_err(|err| anyhow::anyhow!("Error loading cash data: {:?}", err))?;
    let data_credit = mock_select_credit_groupby_label(start, end)
        .map_err(|err| anyhow::anyhow!("Error loading credit data: {:?}", err))?;

    let mut period_map: HashMap<String, HashMap<String, HashMap<i32, f64>>> = HashMap::new();

    // Process cash data
    for entry in data_cash {
        let period_entry = period_map
            .entry(entry.period.clone())
            .or_insert_with(HashMap::new);

        period_entry
            .entry("cash".to_string())
            .or_insert_with(HashMap::new)
            .entry(entry.label_id)
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    // Process credit data
    for entry in data_credit {
        let period_entry = period_map
            .entry(entry.period.clone())
            .or_insert_with(HashMap::new);

        period_entry
            .entry("credit".to_string())
            .or_insert_with(HashMap::new)
            .entry(entry.label_id)
            .and_modify(|e| *e += entry.amount.unwrap_or(0.0))
            .or_insert(entry.amount.unwrap_or(0.0));
    }

    Ok(period_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_data_summary_cash() {
        let start = "2025-01-01";
        let end = "2025-01-31";
        
        // Mock the database function calls
        let data_cash = mock_select_cash_groupby_label(start, end).unwrap();
        let data_credit = mock_select_credit_groupby_label(start, end).unwrap();
    
        // Call the function to get the summary data
        let result = data_sumary_cash(start, end).unwrap();
    
        // Define the expected result structure
        let mut expected_result: HashMap<String, HashMap<String, HashMap<i32, f64>>> = HashMap::new();
        
        // Prepare the cash map
        let mut cash_map: HashMap<i32, f64> = HashMap::new();
        cash_map.insert(1, 100.0);
        cash_map.insert(2, 200.0);
        
        // Insert cash into the expected result
        expected_result.insert("2025-01".to_string(), {
            let mut category_map = HashMap::new();
            category_map.insert("cash".to_string(), cash_map);
            category_map
        });
    
        // Prepare the credit map
        let mut credit_map: HashMap<i32, f64> = HashMap::new();
        credit_map.insert(0, 150.0);
        
        // Insert credit into the existing period
        let mut category_map = expected_result.entry("2025-01".to_string()).or_insert_with(HashMap::new);
        category_map.insert("credit".to_string(), credit_map);
    
        // Assert the result matches the expected output
        println!("Running a test");
        println!("{:?}", expected_result);  // Use {:?} for Debug formatting
        assert_eq!(result, expected_result);
    }
}

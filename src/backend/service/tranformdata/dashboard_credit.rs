use crate::backend::{
    entity::{entity_credit::GroupBySumCredit, entity_label::SelectLabelsName},
    repo::{db_label::db_select::select_labels_name, schema::labels_name::label},
    service::{
        aggr::summary_credit::{data_sumary_credit, sort_label_credit},
        date::period_vec::generate_month_range,
    },
};

pub fn table_period_credit(
    start: &str,
    end: &str,
) -> Result<Vec<(String, Vec<Option<f64>>)>, anyhow::Error> {
    // Step 1: Fetch summarized credit data
    let df: Vec<GroupBySumCredit> = data_sumary_credit(start, end)?;

    // Step 2: Generate the head (sorted labels)
    // let head: Vec<i32> = sort_label_credit(&df)?
    // .iter()
    // .map(|(label_id, _)| *label_id) // Extract label IDs
    // .collect();

    // Filter the data for `show_able == true`
    let mut filtered_data: Vec<SelectLabelsName> = select_labels_name()
        .unwrap()
        .into_iter()
        .filter(|d| d.show_able) // Keep only `show_able == true`
        .collect();

    // Sort the filtered data by `ord`
    let _ = &mut filtered_data.sort_by_key(|d| d.ord);

    // Extract the `id` values from the sorted data
    let head: Vec<i32> = filtered_data.into_iter().map(|d| d.id).collect();

    // Step 3: Generate the row (month range)
    let row: Vec<String> = generate_month_range(start, end).unwrap();

    // Step 4: Create a table sorted by head
    // แก้
    let mut table: Vec<(String, Vec<Option<f64>>)> = row
        .iter()
        .map(|month| {
            let row_data = head
                .iter()
                .map(|label_id| {
                    df.iter()
                        .find(|entry| entry.period == *month && entry.label_id == *label_id) // Match period and label
                        .and_then(|entry| entry.amount) // Extract amount if present
                })
                .collect::<Vec<Option<f64>>>();
            (month.clone(), row_data)
        })
        .collect();

    let first_row = (
        "DATE".to_string(), // Example header label
        head.iter().map(|&label_id| Some(label_id as f64)).collect(), // Convert head IDs to Some(f64)
    );
    table.insert(0, first_row); // Insert at the beginning of the table

    // let last: Vec<f64> = sort_label_credit(&df)?
    // .iter()
    // .map(|(_, amount)| *amount) // Extract amounts from the sorted data
    // .collect();

    // let last_row = (
    //     "TOTAL".to_string(), // Row label
    //     last.iter().map(|&amount| Some(amount)).collect::<Vec<Option<f64>>>() // Convert amounts to Option<f64>
    // );

    // table.push(last_row);

    Ok(table)
}

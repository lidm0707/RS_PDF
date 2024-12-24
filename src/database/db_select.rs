use crate::models::{model_label::*, model_raw::*};
use crate::*;
use anyhow;
use diesel::prelude::*;

pub fn select_labels(
    connection: &mut SqliteConnection,
) -> Result<Vec<SelectLabels>, anyhow::Error> {
    use self::database::schema::labels::dsl::*;

    let results = labels
        .select(SelectLabels::as_select())
        .load(connection)
        .expect("Error loading posts");

    Ok(results)
}

pub fn select_raws(connection: &mut SqliteConnection) -> Result<Vec<SelectRaws>, anyhow::Error> {
    use self::database::schema::raws::dsl::*;

    let results = raws
        .select(SelectRaws::as_select())
        .load(connection)
        .expect("Error loading posts");

    Ok(results)
}

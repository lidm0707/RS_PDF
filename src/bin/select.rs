use rust_pdf::database::{db_connect::connect_database, db_select::select_raws};

fn main() {
    let mut conn = connect_database();
    let display = select_raws(&mut conn).unwrap();
    display.into_iter().for_each(|raw| {
        println!("{:?}", raw); // Ensure SelectRaws implements Debug trait
    });
}

use rust_pdf::backend::repo::db_credit::db_select::select_credit;


fn main() {
    let display = select_credit().unwrap();
    display.into_iter().for_each(|raw| {
        println!("{:?}", raw); // Ensure SelectRaws implements Debug trait
    });
}

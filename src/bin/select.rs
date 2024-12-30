use rust_pdf::database:: db_select::select_raws;

fn main() {
    let display = select_raws().unwrap();
    display.into_iter().for_each(|raw| {
        println!("{:?}", raw); // Ensure SelectRaws implements Debug trait
    });
}

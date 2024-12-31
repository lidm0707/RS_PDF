use anyhow;
use rust_pdf::service::pdf::check_label::search_labels;


fn main() -> Result<(), anyhow::Error> {
    let ctx = "PAYPAL *VPNISE xxxxxx";
    println!("TEST LABEL {:?}",search_labels(ctx));
    Ok(())
}
use rust_pdf::service::pdf::check_label::search_labels;

fn main(){
    println!("{:?}",search_labels("xxx PAYPAL *VPNISE xxx"));
    println!("{:?}",search_labels("xx 09/10"));

}
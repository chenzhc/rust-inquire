use colorize::AnsiColor;
use log::info;

fn main() {
    rust_inquire::init();
    // println!("{}", );
    let message = String::from("Are you ready to proceed?".yellow());

    let proceed = inquire::prompt_confirmation(message);
    if proceed.is_err() {
        info!("Error occurred while checking if to procced");
    }

}

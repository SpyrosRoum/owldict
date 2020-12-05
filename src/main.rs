mod entry;
mod utils;

use std::process::exit;

#[cfg(debug_assertions)]
use dotenv::dotenv;

static OWL_URL: &str = "https://owlbot.info/api/v4/dictionary/";

fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let token = utils::get_token();

    let word = utils::get_word();
    let word_url = OWL_URL.to_owned() + &word;

    let r = ureq::get(&word_url).set("Authorization", &token).call();

    match r.into_json_deserialize::<entry::Entry>() {
        Ok(e) => println!("{}", e),
        Err(_) => {
            eprintln!("I wasn't able to find that word.");
            exit(1);
        }
    };
}

mod entry;

use std::{env, process::exit};

use clap::{App, Arg};
#[cfg(debug_assertions)]
use dotenv::dotenv;

static OWL_URL: &str = "https://owlbot.info/api/v4/dictionary/";

fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let matches = App::new("OwlDict")
        .version("0.1")
        .author("Spyros Roum <spyrosr1@gmail.com>")
        .about("A simple command line dictionary")
        .arg(
            Arg::with_name("word")
                .help("The word to look for")
                .required(true)
                .index(1),
        )
        .get_matches();
    let word = matches.value_of("word").unwrap();
    let token = match env::var("OWLDICT_TOKEN") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Please set your owldict token environment variable.");
            exit(1);
        }
    };
    let token = if !token.starts_with("Token ") {
        "Token ".to_owned() + &token
    } else {
        token
    };

    let word_url = OWL_URL.to_owned() + word;

    let r = ureq::get(&word_url).set("Authorization", &token).call();

    let entry = match r.into_json_deserialize::<entry::Entry>() {
        Ok(e) => e,
        Err(_) => {
            eprintln!("I wasn't able to find that word.");
            exit(1);
        }
    };
    dbg!(&entry);
}

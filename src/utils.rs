use std::{env, process::exit};

use clap::{App, Arg};

pub(crate) fn get_word() -> String {
    let matches = App::new("OwlDict")
        .version("1.0")
        .author("Spyros Roum <spyrosr1@gmail.com>")
        .about("A simple command line dictionary")
        .arg(
            Arg::with_name("word")
                .help("The word to look for")
                .required(true)
                .index(1),
        )
        .get_matches();
    matches.value_of("word").unwrap().to_owned()
}

pub(crate) fn get_token() -> String {
    let token = match env::var("OWLDICT_TOKEN") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Please set your owldict token environment variable.");
            exit(1);
        }
    };
    if !token.starts_with("Token ") {
        "Token ".to_owned() + &token
    } else {
        token
    }
}

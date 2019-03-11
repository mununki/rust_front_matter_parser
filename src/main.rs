extern crate rust_front_matter_parser;

use std::env;
use std::process;

use rust_front_matter_parser::*;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);

        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}

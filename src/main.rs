#[macro_use]
extern crate clap;
extern crate mdfmp;

use clap::{App, Arg};
use std::process;

use mdfmp::*;

fn main() {
    let matches = App::new("Markdown front-matter parser")
        .author("moondaddi <woonki.moon@gmail.com>")
        .version(crate_version!())
        .about("Parse the front-matter data from '*.md' or '*.mdx' files")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Set the type of output")
                .possible_values(&["js", "json"])
                .required(true),
        )
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .value_name("FILE")
                .help("Set the filename of output")
                .required(true),
        )
        .arg(
            Arg::with_name("src")
                .short("s")
                .long("src")
                .value_name("directory")
                .help("Set the source directory")
                .default_value("."),
        )
        .get_matches();

    let config = Config::new(matches).unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);

        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}

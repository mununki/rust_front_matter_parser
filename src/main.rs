extern crate mdfmp;

use std::env;
use std::process;

use mdfmp::*;

fn main() {
    const OPTION_MESSAGE: &'static str =
        "\n\n::: Markdown Front Matter Parser :::\n\n\
         COMMAND: rust_front_matter_parser -t [type] -f [filename] -s [target directory]\n\n\
         -t : (Required) output type (json or js)\t(eg. -t js)\n\
         -f : (Required) output filename\t\t(eg. -f postList)\n\
         -s : (Optional) target directory (default: '.', eg. -s posts)\n";

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);
        eprintln!("{}", OPTION_MESSAGE);

        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        eprintln!("{}", OPTION_MESSAGE);

        process::exit(1);
    };
}

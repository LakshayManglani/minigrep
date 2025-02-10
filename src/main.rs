use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arugments: {}", err);
        process::exit(0);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error running program: {}", e);
        process::exit(0);
    }
}

use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arugments: {}", err);
        process::exit(0);
    });

    println!("> Searching for {}", config.query);
    println!("> In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Error running program: {}", e);
        process::exit(0);
    }
}

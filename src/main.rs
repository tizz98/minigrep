use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    process::exit(match minigrep::run(config) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Application error: {:?}", e);
            1
        }
    })
}

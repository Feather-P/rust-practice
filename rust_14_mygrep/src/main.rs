use std::{env, process};
use rust_14_mygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // So what is a CLOSURE ???
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rust_14_mygrep::run(config){
        eprintln!("Problem occered while running: {e}");
        process::exit(1);
    }
}
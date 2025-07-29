use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() <3 {
            return Err("Not enough arguments")
        }
        let first_str = args[1].clone();
        let second_str = args[2].clone();

        Ok(Config { query: first_str, file_name: second_str })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_name)?;
    println!("{}", &content);
    Ok(())
}
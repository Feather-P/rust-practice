use std::error::Error;
use std::{env, fs};

pub struct Config {
    query: String,
    file_name: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>)
        -> Result<Config, &'static str> {

        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the query argument (first argument)"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the file_name argument (second argument)")
        };
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_name, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_name)?;
    let result = if config.ignore_case {
        search_insensive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    

    for element in result{
        println!("{element}")
    }
    Ok(())
}

pub fn search<'a>(target: &str, content: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines(){
        if line.contains(target){
            result.push(line);
        }
    }
    result
}

pub fn search_insensive<'a>(target: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let target = target.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&target){
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let target = "许下";
        let content = "\
如果你可以肯定 我的一片心意
如果你可以回应 我的一个问题
如果你愿意相信 我们这份默契
就让我们用生命 许下这个约定";
        assert_eq!(vec!["就让我们用生命 许下这个约定"], search(target, content));
    }

    #[test]
    fn insive_search_test() {
        let target = "let";
        let content = "\
Let me hear your voice
Shout it out shout it out
I can feel you
Right here right now";

        assert_eq!(search_insensive(target, content),vec!["Let me hear your voice"]);
    }
}
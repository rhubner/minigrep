use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Result::Err("Not enough arguments");
        }
        Result::Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone()
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {

    let mut result:Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }

    }

    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents)
        );

    }

}
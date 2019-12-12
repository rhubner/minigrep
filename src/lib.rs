use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Result::Err("Not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Result::Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: case_sensitive,
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    let query = query.to_lowercase();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );

    }


    #[test]
    fn case_sensitive() {
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
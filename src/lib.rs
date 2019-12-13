use std::error::Error;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "minigrep",
    about = "Small minigrep, Now with command line parameters!!."
)]
pub struct Opt {
    ///Turn on case insensitive search.
    #[structopt(short, long)]
    pub case_insensitive: bool,

    #[structopt(name = "QUERY")]
    pub query: String,

    #[structopt(name = "FILE")]
    pub filename: String,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

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

pub fn run(config: Opt) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    let results = if !config.case_insensitive {
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

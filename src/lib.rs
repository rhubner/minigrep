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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;
    println!("With text : \n{}", content);
    Ok(())
}
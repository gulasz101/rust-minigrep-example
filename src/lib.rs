use std::fs::read_to_string;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
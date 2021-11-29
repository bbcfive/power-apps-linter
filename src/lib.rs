use std::io;
use std::error::Error;
use std::process;

mod analysis;

pub fn interact() -> Vec<String> {
    println!("Do you want to allow underscore? (y or n)");
    let mut is_underscore_allowed= String::new();
    io::stdin().read_line(&mut is_underscore_allowed).expect("Cannot read line!");

    println!("Do you want to use lower camel case? (y or n)");
    let mut is_lower_camel_case= String::new();
    io::stdin().read_line(&mut is_lower_camel_case).expect("Cannot read line!");

    [is_underscore_allowed, is_lower_camel_case].to_vec()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.command == "lint" {
        if let Err(e) = analysis::analysis_yaml(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    }
    Ok(())
}

pub struct Config {
    pub command: String,
    pub filename: String,
    pub is_underscore_allowed: String,
    pub is_lower_camel_case: String,
}

impl Config {
    pub fn new<'a>(args: &[String], params: &[String]) -> Result<Config, &'a str> {
        let command = if args.len() > 1 { args[1].clone() } else { "lint".to_string() };
        let filename = if args.len() > 2 { args[2].clone() } else { "Empty".to_string() };
        let is_underscore_allowed = params[0].clone();
        let is_lower_camel_case = params[1].clone();
        Ok(Config { command, filename, is_underscore_allowed, is_lower_camel_case })
    }   
}
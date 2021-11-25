use std::error::Error;
use std::process;

mod analysis;

pub fn run(config: Config, is_underscore_allowed: String) -> Result<(), Box<dyn Error>> {
    if config.command == "lint" {
        if let Err(e) = analysis::analysis_yaml(is_underscore_allowed) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    }
    Ok(())
}

pub struct Config {
    pub command: String,
    pub filename: Option<String>,
}

impl Config {
    pub fn new<'a>(args: &[String]) -> Result<Config, &'a str> {
        let command = if args.len() > 1 { args[1].clone() } else { "lint".to_string() };
        let filename = if args.len() > 2 { Some(args[2].clone()) } else { Default::default() };
        Ok(Config { command, filename })
    }   
}
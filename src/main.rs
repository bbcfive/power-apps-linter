use std::env;
use std::process;
use std::io;

use power_apps_linter::Config;
use power_apps_linter::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Do you want to allow underscore? (y or n)");
    let mut is_underscore_allowed= String::new();
    io::stdin().read_line(&mut is_underscore_allowed).expect("Cannot read line!");

    println!("Do you want to use lower camel case? (y or n)");
    let mut is_lower_camel_case= String::new();
    io::stdin().read_line(&mut is_lower_camel_case).expect("Cannot read line!");


    if let Err(e) = run(config, is_underscore_allowed) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}




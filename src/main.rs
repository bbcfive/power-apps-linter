use std::env;
use std::process;

use power_apps_linter::Config;
use power_apps_linter::interact;
use power_apps_linter::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let params: Vec<String> = interact();

    let config = Config::new(&args, &params).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}




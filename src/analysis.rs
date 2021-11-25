use std::fs;
use std::error::Error;

mod rules;
mod helpers;

fn naming_checking(var: &str, allow_underscore: String) -> bool {
    let res = if allow_underscore == "y" { rules::is_camel_case_with_underscore(var) } else { rules::is_camel_case_without_underscore(var)};
    res
}

pub fn analysis_yaml(is_underscore_allowed: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("..\\Screen1.fx.yaml")?;

    let results = helpers::search(" As ", &contents);
    
        for line in results {
            let arr: Vec<&str> = line.trim().split(" ").collect();
            if !naming_checking(arr[0], is_underscore_allowed.trim().to_string()) {
                println!("{} do not meet lowerCamelCase", arr[0]);
            }
        }
    Ok(())
}
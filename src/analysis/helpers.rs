use std::fs::{self, DirEntry};
use std::io::Error;

use crate::Config;

use super::rules;

pub fn get_all_yaml_paths() -> Vec<Result<DirEntry, Error>> {
    let mut results = Vec::new();
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        if path.as_ref().unwrap().path().to_string_lossy().trim().contains(".yaml") {
            results.push(path);
        }
    }

    results
}

pub fn get_query_results<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn naming_checking(var: &str, allow_underscore: String) -> bool {
    let res = if allow_underscore == "y" { rules::is_camel_case_with_underscore(var) } else { rules::is_camel_case_without_underscore(var)};
    res
}

pub fn get_error_warnings(filename: String, configs: &Config ) {
    let path = if filename != "Empty" { filename } else { (*configs.filename).to_string() };
    let contents = fs::read_to_string(path).unwrap();
        let results = get_query_results(" As ", &contents);
        
            for line in results {
                let arr: Vec<&str> = line.trim().split(" ").collect();
                if !naming_checking(arr[0], configs.is_underscore_allowed.trim().to_string()) {
                    println!("{} do not meet lowerCamelCase", arr[0]);
                }
            }
}


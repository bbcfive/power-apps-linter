use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::{
    io::{self, prelude::*, BufReader, Error},
    path::Path,
};

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

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn lint_checking(var: &str, configs: &Config, index: usize, path: String) {
    if configs.is_lower_camel_case.trim().to_string() == "y" {
        if !rules::underscore_checking_with_camel(var, configs.is_underscore_allowed.trim().to_string()) {
            println!("{} do not meet lowerCamelCase in line {}, at {}", var, index + 1, path);
        }
    } else {
        if !rules::underscore_checking_with_pascal(var, configs.is_underscore_allowed.trim().to_string()) {
            println!("{} do not meet upperCamelCase in line {}, at {}", var, index + 1, path);
        }
    }
}

pub fn get_error_warnings(filename: String, configs: &Config ) {
    let path = if filename != "Empty" { filename } else { (*configs.filename).to_string() };
    let filepath = path.clone();
    let contents = lines_from_file(path).expect("Cannot read the file!");
        let mut key_map = HashMap::<&str, String>::new();
        for (i, line) in contents.iter().enumerate() {
            if line.contains(" As ") {
                if i > 1 && contents[i - 1].contains("disable checking for next line") {
                    continue;
                }
                let arr: Vec<&str> = line.trim().split(" ").collect();
                if key_map.contains_key(arr[0]) {
                    println!("{} already exists! It's duplicate in line {}, at {}", arr[0], i + 1, filepath.clone());
                } else {
                    key_map.insert(arr[0], "1".to_string());
                }
                lint_checking(arr[0], configs, i, filepath.clone());
            }
        }
}


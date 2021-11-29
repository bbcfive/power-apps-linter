use std::error::Error;

use crate::Config;

mod rules;
mod helpers;

pub fn analysis_yaml(config: Config) -> Result<(), Box<dyn Error>> {
     if config.filename != "Empty" { 
        helpers::get_error_warnings("Empty".to_string(), &config);
     } else {
        let paths = helpers::get_all_yaml_paths();
        for path in paths {
            helpers::get_error_warnings(path.unwrap().path().to_string_lossy().trim().to_string(), &config);
        }
     }
    Ok(())
}
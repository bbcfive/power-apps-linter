use inflections::case::is_camel_case;

pub fn remove_underscore(var: &str) -> String {
    let res = var.replace("_", "");
    res
}

pub fn is_camel_case_with_underscore(var: &str) -> bool {
    let res = is_camel_case(&remove_underscore(var));
    res
}

pub fn is_camel_case_without_underscore(var: &str) -> bool {
    let res = is_camel_case(var);
    res
}
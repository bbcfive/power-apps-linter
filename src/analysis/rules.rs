use inflections::case::{is_camel_case, is_pascal_case};

fn remove_underscore(var: &str) -> String {
    let res = var.replace("_", "");
    res
}

fn checking_with_underscore(var: &str, lower: bool) -> bool {
    let res = if lower { is_camel_case(&remove_underscore(var)) } else { is_pascal_case(&remove_underscore(var)) };
    res
}

fn checking_without_underscore(var: &str, lower: bool) -> bool {
    let res = if lower { is_camel_case(var) } else { is_pascal_case(var) };
    res
}

pub fn underscore_checking_with_camel(var: &str, allow_underscore: String) -> bool {
    let res = if allow_underscore == "y" { checking_with_underscore(var, true) } else { checking_without_underscore(var, true)};
    res
}

pub fn underscore_checking_with_pascal(var: &str, allow_underscore: String) -> bool {
    let res = if allow_underscore == "y" { checking_with_underscore(var, false) } else { checking_without_underscore(var, false)};
    res
}


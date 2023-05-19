use lua_parser::ast::ParameterList;

pub fn string_list_minification(strings: &Vec<String>) -> String {
    let mut ret = String::new();

    for (i, param) in strings.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += param;
    }

    ret
}

pub fn parameter_list_minification(list: &ParameterList) -> String {
    let mut ret = string_list_minification(&list.names);

    if list.vargs {
        if list.names.len() != 0 {
            ret += ",";
        }
        ret += "...";
    }

    ret
}

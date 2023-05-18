use lua_parser::ast::ParameterList;

pub fn minify_string_list(strings: &Vec<String>) -> String {
    let mut ret = String::new();

    for (i, param) in strings.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += param;
    }

    ret
}

pub fn minify_parameter_list(list: &ParameterList) -> String {
    let mut ret = minify_string_list(&list.names);

    if list.vargs {
        if list.names.len() != 0 {
            ret += ",";
        }
        ret += "...";
    }

    ret
}

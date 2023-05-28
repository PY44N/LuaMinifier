use lua_parser::ast::ParameterList;

use super::variables::get_variable;

pub fn string_list_minification(strings: &Vec<String>) -> String {
    let mut ret = String::new();

    for (i, param) in strings.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += &get_variable(param.to_string());
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

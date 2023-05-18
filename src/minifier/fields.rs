use lua_parser::ast::Field;

use crate::minifier::expressions::minify_expression;

pub fn minify_field(field: &Field) -> String {
    if let Some(key) = &field.key {
        String::from(format!(
            "[{}]={}",
            minify_expression(key),
            minify_expression(&field.val)
        ))
    } else {
        String::from(minify_expression(&field.val))
    }
}

pub fn minify_field_list(fields: &Vec<Field>) -> String {
    let mut ret = String::new();

    for (i, field) in fields.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += &minify_field(field);
    }

    ret
}

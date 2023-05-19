use lua_parser::ast::Field;

use crate::minifier::expressions::expression_minification;

pub fn field_minification(field: &Field) -> String {
    if let Some(key) = &field.key {
        String::from(format!(
            "[{}]={}",
            expression_minification(key),
            expression_minification(&field.val)
        ))
    } else {
        String::from(expression_minification(&field.val))
    }
}

pub fn field_list_minification(fields: &Vec<Field>) -> String {
    let mut ret = String::new();

    for (i, field) in fields.iter().enumerate() {
        if i != 0 {
            ret += ",";
        }
        ret += &field_minification(field);
    }

    ret
}

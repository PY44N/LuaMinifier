use lua_parser::ast::{MethodCall, MethodDefinition};

use crate::minifier::expressions::{minify_expression, minify_expression_list};

pub fn minify_method_definition(method: &MethodDefinition) -> String {
    // TODO: Deal with the fact that this might break things
    // Is there any actual internal difference between : and .
    format!(
        "{}.{}={}",
        minify_expression(&method.receiver),
        method.method,
        minify_expression(&method.body)
    )
}

pub fn minify_method_call(method: &MethodCall) -> String {
    format!(
        "{}.{}({})",
        minify_expression(&method.receiver),
        method.method,
        minify_expression_list(&method.args)
    )
}

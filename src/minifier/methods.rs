use lua_parser::ast::{MethodCall, MethodDefinition};

use crate::minifier::expressions::{expression_list_minification, expression_minification};

pub fn method_definition_minification(method: &MethodDefinition) -> String {
    // TODO: Deal with the fact that this might break things
    // Is there any actual internal difference between : and .
    format!(
        "{}.{}={}",
        expression_minification(&method.receiver),
        method.method,
        expression_minification(&method.body)
    )
}

pub fn method_call_minification(method: &MethodCall) -> String {
    format!(
        "{}.{}({})",
        expression_minification(&method.receiver),
        method.method,
        expression_list_minification(&method.args)
    )
}

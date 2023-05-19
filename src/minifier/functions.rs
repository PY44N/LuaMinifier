use lua_parser::ast::{FunctionCall, FunctionDefinition};

use crate::minifier::expressions::{expression_list_minification, expression_minification};

pub fn function_call_minification(call: &FunctionCall) -> String {
    format!(
        "{}({})",
        expression_minification(&call.func),
        expression_list_minification(&call.args)
    )
}

pub fn function_definition_minification(def: &FunctionDefinition) -> String {
    format!(
        "{} = {}",
        expression_list_minification(&def.name),
        expression_list_minification(&def.body)
    )
}

use lua_parser::ast::{FunctionCall, FunctionDefinition};

use crate::minifier::expressions::{minify_expression, minify_expression_list};

pub fn minify_function_call(call: &FunctionCall) -> String {
    format!(
        "{}({})",
        minify_expression(&call.func),
        minify_expression_list(&call.args)
    )
}

pub fn minify_function_definition(def: &FunctionDefinition) -> String {
    format!(
        "{} = {}",
        minify_expression_list(&def.name),
        minify_expression_list(&def.body)
    )
}

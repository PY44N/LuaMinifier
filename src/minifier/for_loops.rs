use lua_parser::ast::{GenericFor, NumberFor};

use crate::minifier::{
    expressions::{minify_expression, minify_expression_list},
    statements::minify_statement_list, parameters::minify_string_list,
};

pub fn minify_generic_for(for_loop: &GenericFor) -> String {
    format!(
        "for {} in {} do {}end",
        minify_string_list(&for_loop.names),
        minify_expression_list(&for_loop.exprs),
        minify_statement_list(&for_loop.stmts)
    )
}

pub fn minify_numeric_for(for_loop: &NumberFor) -> String {
    // TODO: Some of these arguments are optional
    format!(
        "for {}={},{},{} do {}end",
        for_loop.name,
        minify_expression(&for_loop.init),
        minify_expression(&for_loop.limit),
        minify_expression(&for_loop.step),
        minify_statement_list(&for_loop.stmts)
    )
}

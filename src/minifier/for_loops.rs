use lua_parser::ast::{Expression, GenericFor, NumberFor};

use crate::minifier::{
    expressions::{minify_expression, minify_expression_list},
    parameters::minify_string_list,
    statements::minify_statement_list,
};

pub fn minify_generic_for(for_loop: &GenericFor) -> String {
    format!(
        "for {} in {} do {}end",
        minify_string_list(&for_loop.names),
        minify_expression_list(&for_loop.exprs),
        minify_statement_list(&for_loop.stmts)
    )
}

// I hate this function name
fn is_default_numeric_for_loop_step(for_loop: &NumberFor) -> bool {
    if let Expression::Number(num) = for_loop.step {
        return num == 1.0;
    }
    false
}

pub fn minify_numeric_for(for_loop: &NumberFor) -> String {
    if is_default_numeric_for_loop_step(for_loop) {
        format!(
            "for {}={},{} do {}end",
            for_loop.name,
            minify_expression(&for_loop.init),
            minify_expression(&for_loop.limit),
            minify_statement_list(&for_loop.stmts)
        )
    } else {
        format!(
            "for {}={},{},{} do {}end",
            for_loop.name,
            minify_expression(&for_loop.init),
            minify_expression(&for_loop.limit),
            minify_expression(&for_loop.step),
            minify_statement_list(&for_loop.stmts)
        )
    }
}

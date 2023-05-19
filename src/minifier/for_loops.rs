use lua_parser::ast::{Expression, GenericFor, NumberFor};

use crate::minifier::{
    expressions::{expression_list_minification, expression_minification},
    parameters::string_list_minification,
    statements::statement_list_minification,
};

pub fn generic_for_minification(for_loop: &GenericFor) -> String {
    format!(
        "for {} in {} do {}end",
        string_list_minification(&for_loop.names),
        expression_list_minification(&for_loop.exprs),
        statement_list_minification(&for_loop.stmts)
    )
}

// I hate this function name
fn is_default_numeric_for_loop_step(for_loop: &NumberFor) -> bool {
    if let Expression::Number(num) = for_loop.step {
        return num == 1.0;
    }
    false
}

pub fn numeric_for_minification(for_loop: &NumberFor) -> String {
    if is_default_numeric_for_loop_step(for_loop) {
        format!(
            "for {}={},{} do {}end",
            for_loop.name,
            expression_minification(&for_loop.init),
            expression_minification(&for_loop.limit),
            statement_list_minification(&for_loop.stmts)
        )
    } else {
        format!(
            "for {}={},{},{} do {}end",
            for_loop.name,
            expression_minification(&for_loop.init),
            expression_minification(&for_loop.limit),
            expression_minification(&for_loop.step),
            statement_list_minification(&for_loop.stmts)
        )
    }
}

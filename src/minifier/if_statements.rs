use lua_parser::ast::IfThenElse;

use crate::minifier::{expressions::minify_expression, statements::minify_statement_list};

pub fn minify_if(if_statement: &IfThenElse) -> String {
    format!(
        "if {} then {}else {}end",
        minify_expression(&if_statement.condition),
        minify_statement_list(&if_statement.then),
        minify_statement_list(&if_statement.els),
    )
}

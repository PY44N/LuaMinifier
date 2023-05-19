use lua_parser::ast::IfThenElse;

use crate::minifier::{
    expressions::expression_minification, statements::statement_list_minification,
};

pub fn if_minification(if_statement: &IfThenElse) -> String {
    if if_statement.els.len() == 0 {
        format!(
            "if {} then {}end",
            expression_minification(&if_statement.condition),
            statement_list_minification(&if_statement.then),
        )
    } else {
        format!(
            "if {} then {}else {}end",
            expression_minification(&if_statement.condition),
            statement_list_minification(&if_statement.then),
            statement_list_minification(&if_statement.els),
        )
    }
}

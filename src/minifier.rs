use lua_parser::ast::Statement;

use self::statements::minify_statement_list;

mod expressions;
mod fields;
mod for_loops;
mod functions;
mod if_statements;
mod methods;
mod operators;
mod parameters;
mod statements;

pub fn minify_ast(ast: Vec<Statement>) -> String {
    minify_statement_list(&ast)
}

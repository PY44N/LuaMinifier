use lua_parser::ast::Statement;

use self::statements::statement_list_minification;

mod expressions;
mod fields;
mod for_loops;
mod functions;
mod if_statements;
mod methods;
mod operators;
mod parameters;
mod statements;
mod util;
mod variables;

pub fn minify_ast(ast: Vec<Statement>) -> String {
    statement_list_minification(&ast)
}

use lua_parser::{parser::ParserResult, state::State, Result};

use self::{statements::statement_list_minification, variables::load_locals};

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

pub fn minify_ast(parser_result: ParserResult) -> String {
    load_locals(parser_result.locals);

    statement_list_minification(&parser_result.ast)
}

pub fn minify(file: &str) -> Result<String> {
    let mut state = State::new();
    let parser_result = state.parse_file(file)?;

    Ok(minify_ast(parser_result))
}

use std::{fs, path::Path};

use lua_minifier::minifier::minify_ast;
use lua_parser::state::State;

fn main() {
    let mut state = State::new();
    let parser_result = state.parse_file("input.lua").expect("Failed to parse file");

    let output = minify_ast(parser_result);
    println!("{}", output);
    if Path::new("output.lua").exists() {
        fs::remove_file("output.lua").unwrap();
    }
    fs::write("output.lua", output).unwrap();
}

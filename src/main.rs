use std::{fs, path::Path};

use lua_parser::state::State;
use minifier::Minifier;

pub mod minifier;

fn main() {
    let mut state = State::new();
    let ast = state.parse_file("input.lua").expect("Failed to parse file");

    println!("{:?}", ast);

    let mut minifier = Minifier::new(ast);
    let output = minifier.minify();
    println!("{}", output);
    if Path::new("output.lua").exists() {
        fs::remove_file("output.lua").unwrap();
    }
    fs::write("output.lua", output).unwrap();
}

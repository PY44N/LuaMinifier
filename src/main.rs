use std::{fs, path::Path};

use lua_minifier::minifier::minify;
use lua_parser::state::State;

fn main() {
    let output = minify("input.lua").expect("Failed to minify");
    println!("{}", output);
    if Path::new("output.lua").exists() {
        fs::remove_file("output.lua").unwrap();
    }
    fs::write("output.lua", output).unwrap();
}

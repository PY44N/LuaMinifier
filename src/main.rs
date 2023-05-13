use lua_parser::state::State;
use minifier::Minifier;

pub mod minifier;

fn main() {
    let mut state = State::new();
    let ast = state.parse_file("input.lua").expect("Failed to parse file");

    let mut minifier = Minifier::new(ast);
    minifier.minify();
}

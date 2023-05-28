# Lua Minfier
A basic lua minification program written in Rust

## Usage

Add the following to your *Cargo.toml* file add the depdendancy to your project
```toml
lua_minifier = { git = "https://github.com/PY44N/LuaMinifier" }
```

Use the following code to minify lua code from a file

```rust
let output = minify("file.lua").expect("Failed to minify");
println!("{}", output);
```

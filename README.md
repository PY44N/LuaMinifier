# Lua Minfier
A basic Lua minification program written in pure Rust

Created for an upcoming Lua obfuscator project

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

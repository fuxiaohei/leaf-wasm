[package]
name = "{{name}}"
version = "0.1.0"
authors = []
edition = "2021"

[dependencies]
anyhow = "1.0.66"
bytes = "1.3.0"
http = "0.2.8"
leaf-sdk = { git = "https://github.com/fuxiaohei/leaf-wasm" }
leaf-sdk-macro = { git = "https://github.com/fuxiaohei/leaf-wasm" }
wit-bindgen-guest-rust = { git = "https://github.com/bytecodealliance/wit-bindgen" }

[lib]
crate-type = ["cdylib"]

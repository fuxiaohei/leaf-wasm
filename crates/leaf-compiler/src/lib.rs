use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use wit_parser::World;

pub fn parse_world(s: &str) -> Result<World> {
    let path = Path::new(s);
    if !path.is_file() {
        panic!("wit file `{}` does not exist", path.display());
    }

    let world = World::parse_file(path)
        .with_context(|| format!("failed to parse wit file `{}`", path.display()))
        .map_err(|e| {
            eprintln!("{e:?}");
            e
        })?;

    Ok(world)
}

/// Generate rust guest code
pub fn gen_rust_guest_code(fpath: &str) -> Result<(String, String)> {
    // get path base name and set extension to rs
    let file_name = Path::new(fpath).file_name().unwrap().to_str().unwrap();
    let mut rs_path = PathBuf::from(file_name);
    rs_path.set_extension("rs");
    let rs_path = rs_path.to_str().unwrap();
    let target_rs_file = Path::new(fpath).with_file_name(rs_path);

    let world = parse_world(fpath).unwrap();
    let opts = wit_bindgen_gen_guest_rust::Opts {
        rustfmt: true,
        macro_export: true,
        ..Default::default()
    };
    let mut builder = opts.build();
    let mut files = wit_bindgen_core::Files::default();

    builder.generate(&world, &mut files);
    for (name, contents) in files.iter() {
        if name == rs_path {
            return Ok((
                target_rs_file.to_str().unwrap().to_string(),
                String::from_utf8_lossy(contents).to_string(),
            ));
        }
    }
    Err(anyhow::anyhow!("{} not found in generator", rs_path))
}

/// Generate js host code
pub fn gen_js_host_code(fpath: &str) -> Result<(String, String)> {
    let file_name = Path::new(fpath).file_name().unwrap().to_str().unwrap();
    let mut rs_path = PathBuf::from(file_name);
    rs_path.set_extension("js");
    let rs_path = rs_path.to_str().unwrap();
    let target_rs_file = Path::new(fpath).with_file_name(rs_path);

    let world = parse_world(fpath).unwrap();
    let opts = wit_bindgen_gen_host_js::Opts::default();
    let mut builder = opts.build().unwrap();
    let mut files = wit_bindgen_core::Files::default();
    builder.generate(&world, &mut files);
    // FIXME: it generates *.d.ts
    for (name, contents) in files.iter() {
        println!(
            " file {}, content:\n{}",
            name,
            String::from_utf8_lossy(contents)
        );
        if name == rs_path {
            return Ok((
                target_rs_file.to_str().unwrap().to_string(),
                String::from_utf8_lossy(contents).to_string(),
            ));
        }
    }
    Err(anyhow::anyhow!("{} not found in generator", rs_path))
}

mod compile;

pub use compile::compile_js;
pub use compile::compile_rust;
pub use compile::encode_wasm_component;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use wit_bindgen_gen_guest_rust::Opts;
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

pub fn guest_rust_code_gen(fpath: &str) -> Result<(String, String)> {
    // get path base name and set extension to rs
    let file_name = Path::new(fpath).file_name().unwrap().to_str().unwrap();
    let mut rs_path = PathBuf::from(file_name);
    rs_path.set_extension("rs");
    let rs_path = rs_path.to_str().unwrap();
    let target_rs_file = Path::new(fpath).with_file_name(rs_path);

    let world = parse_world(fpath).unwrap();
    let opts = Opts {
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

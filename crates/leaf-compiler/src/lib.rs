use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::path::Path;
use wit_bindgen_core::{Files, WorldGenerator};
use wit_parser::{Resolve, UnresolvedPackage};

pub enum GuestGeneratorType {
    Rust,
    Js,
    Golang,
}

/// parse wit file and return world id
pub fn generate_world_guest(
    s: &str,
    world: Option<String>,
    t: GuestGeneratorType,
) -> Result<HashMap<String, String>> {
    // parse exported world in wit file
    let path = Path::new(s);
    if !path.is_file() {
        panic!("wit file `{}` does not exist", path.display());
    }

    let mut resolve = Resolve::default();
    let pkg = resolve.push(UnresolvedPackage::parse_file(path)?, &Default::default())?;

    let world = match &world {
        Some(world) => {
            let mut parts = world.splitn(2, '.');
            let doc = parts.next().unwrap();
            let world = parts.next();
            let doc = *resolve.packages[pkg]
                .documents
                .get(doc)
                .ok_or_else(|| anyhow!("no document named `{doc}` in package"))?;
            match world {
                Some(name) => *resolve.documents[doc]
                    .worlds
                    .get(name)
                    .ok_or_else(|| anyhow!("no world named `{name}` in document"))?,
                None => resolve.documents[doc]
                    .default_world
                    .ok_or_else(|| anyhow!("no default world in document"))?,
            }
        }
        None => {
            let mut docs = resolve.packages[pkg].documents.iter();
            let (_, doc) = docs
                .next()
                .ok_or_else(|| anyhow!("no documents found in package"))?;
            if docs.next().is_some() {
                bail!("multiple documents found in package, specify a default world")
            }
            resolve.documents[*doc]
                .default_world
                .ok_or_else(|| anyhow!("no default world in document"))?
        }
    };

    // get guest genrator
    let mut generator = gen_guest_code_builder(t)?;

    // generate file
    let mut files = Files::default();
    generator.generate(&resolve, world, &mut files);

    let mut output_maps = HashMap::new();
    for (name, contents) in files.iter() {
        output_maps.insert(
            name.to_string(),
            String::from_utf8_lossy(contents).to_string(),
        );
    }
    Ok(output_maps)
}

/// Generate guest code builder
fn gen_guest_code_builder(t: GuestGeneratorType) -> Result<Box<dyn WorldGenerator>> {
    match t {
        GuestGeneratorType::Rust => {
            let opts = wit_bindgen_gen_guest_rust::Opts {
                macro_export: true,
                rustfmt: true,
                ..Default::default()
            };
            let builder = opts.build();
            Ok(builder)
        }
        _ => Err(anyhow::anyhow!("not support guest generator")),
    }
}

/*
/// Generate rust guest code
pub fn gen_rust_guest_code(fpath: &str) -> Result<(String, String)> {
    // get path base name and set extension to rs
    let file_name = Path::new(fpath).file_name().unwrap().to_str().unwrap();
    let mut rs_path = PathBuf::from(file_name);
    rs_path.set_extension("rs");
    let rs_path = rs_path.to_str().unwrap();
    let target_rs_file = Path::new(fpath).with_file_name(rs_path);

    let world = parse_world(fpath, Some("abc".to_string())).unwrap();
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
}*/

mod compile;

pub use compile::compile_js;
pub use compile::compile_rust;
pub use compile::encode_wasm_component;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_rust_guest() {
        let files = generate_world_guest("./wit/strings.wit", None, GuestGeneratorType::Rust);
        assert!(files.unwrap().contains_key(&"the-world.rs".to_string()));
    }
}

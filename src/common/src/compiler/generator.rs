use anyhow::{anyhow, Result,bail};
use std::collections::HashMap;
use std::path::Path;
use wit_bindgen_core::{Files, WorldGenerator};
use wit_parser::{Resolve, UnresolvedPackage};

pub enum GuestGeneratorType {
    Rust,
    Js,
    Golang,
}

/// generate_world_guest generate world guest code
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

    // prepare file resovler
    let mut resolve = Resolve::default();
    let pkg = resolve.push(UnresolvedPackage::parse_file(path)?, &Default::default())?;

    // parse world or use default world
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

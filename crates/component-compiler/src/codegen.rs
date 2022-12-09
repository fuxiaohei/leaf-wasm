use anyhow::Context;
use anyhow::Result;
use std::path::Path;
use wit_bindgen_gen_guest_rust::Opts;
use wit_parser::World;

fn parse_world(s: &str) -> Result<World> {
    let path = Path::new(s);
    if !path.is_file() {
        panic!("wit file `{}` does not exist", path.display());
    }

    let world = World::parse_file(&path)
        .with_context(|| format!("failed to parse wit file `{}`", path.display()))
        .map_err(|e| {
            eprintln!("{e:?}");
            e
        })?;

    Ok(world)
}

pub fn code_gen(s: &str) -> Result<String> {
    let world = parse_world(s).unwrap();
    let opts = Opts::default();
    let mut builder = opts.build();
    let mut files = wit_bindgen_core::Files::default();
    builder.generate(&world, &mut files);
    for (name, contents) in files.iter() {
        if name == "leaf-http.rs" {
            return Ok(String::from_utf8_lossy(contents).to_string());
        }
    }
    Err(anyhow::anyhow!("leaf-http.rs not found"))
}

#[test]
fn run_codegen_test() {
    let res = code_gen("../../wit/leaf-http.wit");
    assert!(res.is_ok())
}

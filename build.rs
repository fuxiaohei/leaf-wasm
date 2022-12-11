use component_compiler::guest_rust_code_gen;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wit/*.wit");

    let mut config = vergen::Config::default();
    *config.git_mut().sha_kind_mut() = vergen::ShaKind::Short;
    *config.git_mut().commit_timestamp_kind_mut() = vergen::TimestampKind::DateOnly;
    vergen::vergen(config).expect("failed to extract build information");

    build_wit_guest_code();
}

fn build_wit_guest_code() {
    // loop wit directory, find .wit files , check same name as .rs file, if not, generate it
    let wit_dir = Path::new("./wit");
    let wit_files = wit_dir.read_dir().unwrap();
    for wit_file in wit_files {
        let wit_file_path = wit_file.unwrap().path();
        if !wit_file_path.is_file() {
            continue;
        }
        if wit_file_path.extension().unwrap() != "wit" {
            continue;
        }
        let (target_rs, target_rs_content) =
            guest_rust_code_gen(wit_file_path.to_str().unwrap()).unwrap();
        std::fs::write(target_rs, target_rs_content).unwrap();
    }
}

use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let mut prost_build = prost_build::Config::new();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    prost_build.out_dir(out_dir);
    prost_build
        .compile_protos(&["protos/authz_for_osmosis.proto"], &["protos/"])
        .unwrap();

    // Trippy thing GPT told me when we were hangin'
    println!("cargo:rerun-if-changed=protos/");

    let out_dir = env::var("OUT_DIR").unwrap();

    // Path of original generated file, deep in the target directory
    // We'll copy this into the project later
    let generated_file = Path::new(&out_dir).join("authz_for_osmosis.rs");

    // Generated file useful in CosmWasm contracts to do deeper, Stargate stuff
    let destination_file = Path::new("protos-in-rust").join("authz_for_osmosis.rs");

    // Copy the generated file to a sane place. Serenity now.
    fs::copy(generated_file, destination_file).unwrap();
}

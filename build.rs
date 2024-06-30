use std::error::Error;
use std::{self, path::Path};
use xnor_codegen::{ranks, tensor_from_array};

const MAX_NDIM: usize = 9;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir =
        std::env::var_os("OUT_DIR").expect("build script should generate output directory");
    let path = Path::new(&out_dir).join("ranks.rs");
    std::fs::write(path, ranks::generate_code(MAX_NDIM))?;
    let path = Path::new(&out_dir).join("tensor_from_array.rs");
    std::fs::write(path, tensor_from_array::generate_code(MAX_NDIM))?;
    //println!("cargo::rerun-if-changed=build.rs");
    Ok(())
}

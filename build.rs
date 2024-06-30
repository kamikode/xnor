use std::error::Error;
use std::{self, path::Path};
use walkdir::WalkDir;
use xnor_codegen::{ranks, ranks_import, tensor_from_array};

const MAX_NDIM: usize = 9;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir =
        std::env::var_os("OUT_DIR").expect("build script should generate output directory");
    let path = Path::new(&out_dir).join("ranks.rs");
    std::fs::write(path, ranks::generate_code(MAX_NDIM))?;
    let path = Path::new(&out_dir).join("ranks_import.rs");
    std::fs::write(path, ranks_import::generate_code(MAX_NDIM))?;
    let path = Path::new(&out_dir).join("tensor_from_array.rs");
    std::fs::write(path, tensor_from_array::generate_code(MAX_NDIM))?;
    //

    // Trigger recompilation if any file in xnor-codegen (or build.rs) changes.
    println!("cargo::rerun-if-changed=build.rs");
    for entry in WalkDir::new("xnor-codegen/")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let path = String::from(entry.path().to_string_lossy());
        if path.ends_with(".rs") {
            println!("cargo::rerun-if-changed={}", path);
        }
    }

    Ok(())
}

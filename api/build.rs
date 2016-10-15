extern crate serde_codegen;
extern crate diesel_codegen_syntex as diesel_codegen;

use std::env;
use std::path::Path;

#[cfg(feature = "with-syntex")]
fn main()
{
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");

    // Take care of Diesel config
    let mut registry = syntex::Registry::new();
    diesel_codegen::register(&mut registry);

    let src_diesel = Path::new("src/lib.in.rs");
    let dst_diesel = Path::new(&out_dir).join("lib.rs");

    registry.expand("", &src, &dst).unwrap();
    
    serde_codegen::expand(&src, &dst).unwrap();
}

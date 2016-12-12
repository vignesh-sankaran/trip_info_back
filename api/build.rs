extern crate serde_codegen;
extern crate syntex;
extern crate diesel_codegen_syntex as diesel_codegen;

use std::env;
use std::path::Path;

fn main()
{
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");

    serde_codegen::expand(&src, &dst).unwrap();

    let out_dir_diesel = env::var_os("OUT_DIR").unwrap();
    let mut registry = syntex::Registry::new();
    diesel_codegen::register(&mut registry);

    let src_diesel = Path::new("src/lib.in.rs");
    let dst_diesel = Path::new(&out_dir_diesel).join("lib.rs");

    registry.expand("", &src_diesel, &dst_diesel);
}
use std::env;
use std::path::PathBuf;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("blog_descriptor.bin"))
        .compile(&["proto/blog.proto"], &["proto"])?;

    println!("cargo:rerun-if-changed=proto/blog.proto");
    Ok(())
}

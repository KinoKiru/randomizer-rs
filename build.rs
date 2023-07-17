use std::path::PathBuf;
use std::{env, io};

use walkdir::WalkDir;
#[cfg(windows)]
use winres::WindowsResource;

fn main() -> io::Result<()> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("descriptor.bin");

    let protos = WalkDir::new("proto")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let is_proto = entry.file_name().to_string_lossy().ends_with(".proto");
            if is_proto {
                Some(entry.into_path())
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>();

    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .build_client(false)
        .build_server(true)
        .compile(protos.as_slice(), &["proto/"])?;

    #[cfg(windows)]
    {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon_with_id("icon.ico", "2")
            .compile()?;
    }
    Ok(())
}

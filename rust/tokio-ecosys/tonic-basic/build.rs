// use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        // .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .out_dir("src/pb")
        .compile(
            &[
                // "protos/types.proto",
                "protos/gretter.proto",
                "protos/gretter2.proto",
            ],
            &["protos"],
        )
        .unwrap();

    println!("cargo:rerun-if-changed=protos/types.proto");
    println!("cargo:rerun-if-changed=protos/gretter.proto");
    println!("cargo:rerun-if-changed=protos/gretter2.proto");

    Ok(())
}

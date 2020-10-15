extern crate protobuf_codegen_pure;
extern crate reqwest;

use protobuf_codegen_pure::{Codegen, Customize};
use reqwest::blocking::get;
use std::env;
use std::fs;
use std::path::Path;

const SCHEMA_URL: &str = "https://raw.githubusercontent.com/hypercore-protocol/hyperspace-rpc/3160f65d36fb764852bc96716906941df76df555/schema.proto";

fn main() {
    // Prepare paths
    let current_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Cargo should have set the CARGO_MANIFEST_DIR environment variable");
    let output_dir = format!("{}/src/proto", &current_dir);
    // if !Path::new(&output_dir).exists() {
    //     fs::create_dir(&output_dir).expect("Could not create proto output dir");
    // }
    let schema_proto_path = format!("{}/schema.proto", &current_dir);
    if Path::new(&schema_proto_path).exists() {
        fs::remove_file(&schema_proto_path).expect("Could not remove pre-existing output file");
    }
    let hrpc_proto_path = format!("{}/hrpc.proto", &current_dir);

    // Download schema.proto
    let mut schema_res =
        get(SCHEMA_URL).expect("Could not download schema.proto from hyperspace-rpc");
    if Path::new(&schema_proto_path).exists() {
        fs::remove_file(&schema_proto_path).expect("Could not remove pre-existing output file");
    }
    let mut output_file =
        std::fs::File::create(&schema_proto_path).expect("Could not create output file");
    schema_res.copy_to(&mut output_file).unwrap();

    // Run generator
    Codegen::new()
        .out_dir(&output_dir)
        .inputs(&[&schema_proto_path, &hrpc_proto_path])
        .include(&current_dir)
        .customize(Customize {
            carllerche_bytes_for_bytes: Some(true),
            carllerche_bytes_for_string: Some(true),
            ..Default::default()
        })
        .run()
        .expect("protoc");
}

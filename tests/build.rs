use actix_prost_build::{ActixGenerator, GeneratorList};
use prost_build::{Config, ServiceGenerator};
use std::path::Path;

// custom function to include custom generator
fn compile(
    protos: &[impl AsRef<Path>],
    includes: &[impl AsRef<Path>],
    generator: Box<dyn ServiceGenerator>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config
        .service_generator(generator)
        .out_dir("src/proto")
        .protoc_arg("--openapiv2_out=proto")
        .protoc_arg("--openapiv2_opt")
        .protoc_arg("grpc_api_configuration=proto/http_api.yaml,output_format=yaml")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    for path in protos.iter() {
        println!("cargo:rerun-if-changed={}", path.as_ref().display())
    }

    for path in includes.iter() {
        // Cargo will watch the **entire** directory recursively. If we
        // could figure out which files are imported by our protos we
        // could specify only those files instead.
        println!("cargo:rerun-if-changed={}", path.as_ref().display())
    }

    config.compile_protos(protos, includes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gens = Box::new(GeneratorList::new(vec![
        tonic_build::configure().service_generator(),
        Box::new(ActixGenerator::new("proto/http_api.yaml").unwrap()),
    ]));
    compile(
        &["proto/rest.proto", "proto/types.proto"],
        &["proto/", "proto/googleapis", "proto/grpc-gateway"],
        gens,
    )?;
    Ok(())
}

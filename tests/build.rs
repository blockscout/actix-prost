use actix_prost_build::{ActixGenerator, GeneratorList};
use prost_build::{Config, ServiceGenerator};
use std::{
    env,
    path::{Path, PathBuf},
};

// custom function to include custom generator
fn compile(
    protos: &[impl AsRef<Path>],
    includes: &[impl AsRef<Path>],
    generator: Box<dyn ServiceGenerator>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config
        .service_generator(generator)
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin"),
        )
        .out_dir("src/proto")
        .bytes(["."])
        .compile_well_known_types()
        .protoc_arg("--openapiv2_out=proto")
        .protoc_arg("--openapiv2_opt")
        .protoc_arg("grpc_api_configuration=proto/http_api.yaml,output_format=yaml")
        .type_attribute(".conversions", "#[actix_prost_macros::serde]")
        .type_attribute(".errors", "#[actix_prost_macros::serde]")
        .type_attribute(".rest", "#[actix_prost_macros::serde]")
        .type_attribute(".simple", "#[actix_prost_macros::serde]")
        .type_attribute(".types", "#[actix_prost_macros::serde]")
        .type_attribute(
            ".snake_case_types",
            "#[actix_prost_macros::serde(rename_all=\"snake_case\")]",
        );

    config.compile_protos(protos, includes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gens = Box::new(GeneratorList::new(vec![
        tonic_build::configure().service_generator(),
        Box::new(ActixGenerator::new("proto/http_api.yaml").unwrap()),
    ]));
    compile(
        &[
            "proto/rest.proto",
            "proto/simple.proto",
            "proto/types.proto",
            "proto/errors.proto",
            "proto/conversions.proto",
            "proto/snake_case_types.proto",
        ],
        &["proto/", "proto/googleapis", "proto/grpc-gateway"],
        gens,
    )?;
    Ok(())
}

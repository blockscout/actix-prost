# actix-prost

Generate actix handles and routes with ease!

## Usage

You can see some examples in [tests](tests/) crate

### Limitations

Currently, due to lack of `prost` support for custom extensions, `google.api.http` options are not supported in proto files.

Instead, you can write [`.yaml` api](https://cloud.google.com/endpoints/docs/grpc/transcoding#configuring_transcoding_in_yaml), which we can read and parse, and provide it in `build.rs`

### Steps

Add this to `Cargo.toml`

```toml
[dependencies]
# we're not released yet
actix-prost = { git = "https://github.com/blockscout/actix-prost" }
actix-web = "4"
serde = { version = "1", features = ["derive"] }
async-trait = "0.1"
prost = "0.11"
tonic = "0.8"

[build-dependencies]
actix-prost-build = { git = "https://github.com/blockscout/actix-prost" }
tonic-build = "0.8"
prost-build = "0.11"
```

And add this to `build.rs`

```rust
use actix_prost_build::{ActixGenerator, GeneratorList};
use prost_build::{Config, ServiceGenerator};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = "path/to/api.yaml";
    let gens = Box::new(GeneratorList::new(vec![
        // tonic generator is required, because we need it's trait
        tonic_build::configure().service_generator(),
        // actix generator
        Box::new(ActixGenerator::new(api).unwrap()),
    ]));
    let mut config = Config::new();
    config
        .service_generator(generator)
        // this is not required, but it will force protoc to check that yaml is valid
        .protoc_arg(format!("grpc_api_configuration={},output_format=yaml", api))
        // this is required
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.compile_protos(protos, includes)?;
    Ok(())
}
```

## What's here and what's not

> ✔️ = done, ⌛ = will be done soon, ❌ = not planned

 * ✔️ Generate custom handlers, which calls `gRPC` implementation
 * ✔️ Use `path`, `query` and `json` extractors
 * ✔️ Return `json` encoded response
 * ✔️ Use recommended mapping from `gRPC` codes to `http` codes
 * ✔️ Generate router which will route all the handlers
 * ⌛ Pass headers into `tonic::Request`
 * ⌛ Map response using `response_body`
 * ⌛ Support `google.api.http` options inside proto files (as soon, as `prost` will support them)
 * ❌ Use all the features from `gRPC` path option
 * ❌ `http` client implementation

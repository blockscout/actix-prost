use crate::{
    proto::types::{
        types_rpc_actix::route_types_rpc, types_rpc_server::TypesRpc, Complex, Enums, Maps, OneOfs,
        OptionalEnums, OptionalScalars, Repeated, Scalars,
    },
    test,
};
use actix_web::{App, HttpServer};
use pretty_assertions::assert_eq;
use std::{net::SocketAddr, sync::Arc};
use tonic::{Request, Response, Status};

#[derive(Default)]
struct TypesServer {}

#[async_trait::async_trait]
impl TypesRpc for TypesServer {
    async fn scalars_rpc(&self, request: Request<Scalars>) -> Result<Response<Scalars>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn optional_scalars_rpc(
        &self,
        request: Request<OptionalScalars>,
    ) -> Result<Response<OptionalScalars>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn enums_rpc(&self, request: Request<Enums>) -> Result<Response<Enums>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn optional_enums_rpc(
        &self,
        request: Request<OptionalEnums>,
    ) -> Result<Response<OptionalEnums>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn repeated_rpc(&self, request: Request<Repeated>) -> Result<Response<Repeated>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn maps_rpc(&self, request: Request<Maps>) -> Result<Response<Maps>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn one_ofs_rpc(&self, request: Request<OneOfs>) -> Result<Response<OneOfs>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn complex_rpc(&self, request: Request<Complex>) -> Result<Response<Complex>, Status> {
        Ok(Response::new(request.into_inner()))
    }
}

async fn assert_ping(addr: &SocketAddr, path: &str, body: String) {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://localhost:{}{}", addr.port(), path))
        .body(body.clone())
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp)
        .unwrap_or_else(|_| panic!("could not parse json, got: {}", resp));
    assert_eq!(body, resp);
}

#[tokio::test]
async fn ping() {
    let server = Arc::new(TypesServer::default());
    let addr: SocketAddr = test::get_test_addr();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_types_rpc(config, server.clone()))
    })
    .bind(addr)
    .unwrap();

    tokio::spawn(http.run());

    assert_ping(
        &addr,
        "/types/scalars", 
        r#"{"a":123.0,"b":"1000000000000000000","c":"hello world","d":"dGhpcyBpcyBiYXNlNjQgZW5jb2RlZCBzdHJpbmc=","e":true}"#.into(),
    )
    .await;
    assert_ping(
        &addr,
        "/types/optional_scalars", 
        r#"{"a":123.0,"b":"1000000000000000000","c":"hello world","d":"dGhpcyBpcyBiYXNlNjQgZW5jb2RlZCBzdHJpbmc=","e":true}"#.into(),
    )
    .await;
    assert_ping(
        &addr,
        "/types/optional_scalars",
        r#"{"a":null,"b":null,"c":null,"d":null,"e":null}"#.into(),
    )
    .await;
    assert_ping(&addr, "/types/enums", r#"{"values":"BAR"}"#.into()).await;
    assert_ping(&addr, "/types/optional_enums", r#"{"values":"BAR"}"#.into()).await;
    assert_ping(&addr, "/types/optional_enums", r#"{"values":null}"#.into()).await;
    assert_ping(&addr, "/types/optional_enums", r#"{}"#.into()).await;
    assert_ping(
        &addr,
        "/types/repeated",
        r#"{"foo":["foo","bar","baz"]}"#.into(),
    )
    .await;
    assert_ping(
        &addr,
        "/types/maps",
        r#"{"foo":{"bar":432,"baz":12345,"foo":123}}"#.into(),
    )
    .await;
    assert_ping(&addr, "/types/oneofs", r#"{"foo":"hello world"}"#.into()).await;
    assert_ping(
        &addr,
        "/types/oneofs",
        r#"{"bar":"dGhpcyBpcyBiYXNlNjQgZW5jb2RlZCBzdHJpbmc="}"#.into(),
    )
    .await;
    assert_ping(&addr, "/types/oneofs", r#"{"baz":"12345"}"#.into()).await;
    assert_ping(
        &addr,
        "/types/complex", 
        r#"{"enums":{"values":"BAR"},"maps":{"foo":{"foo":123,"bar":432,"baz":12345}},"oneofs":{"foo":"hello world"},"repeated":{"foo":["foo","bar","baz"]},"scalars":{"a":123.0,"b":"1000000000000000000","c":"hello world","d":"dGhpcyBpcyBiYXNlNjQgZW5jb2RlZCBzdHJpbmc=","e":true}}"#.into(),
    )
    .await;
}

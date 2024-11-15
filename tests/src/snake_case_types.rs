use crate::{
    proto::snake_case_types::{
        snake_case_types_rpc_actix::route_snake_case_types_rpc,
        snake_case_types_rpc_server::SnakeCaseTypesRpc, OneOfs, SimpleMessages,
    },
    test,
};
use actix_web::{App, HttpServer};
use pretty_assertions::assert_eq;
use std::{net::SocketAddr, sync::Arc};
use tonic::{Request, Response, Status};

#[derive(Default)]
struct SnakeCaseTypesServer {}

#[async_trait::async_trait]
impl SnakeCaseTypesRpc for SnakeCaseTypesServer {
    async fn simple_messages_rpc(
        &self,
        request: Request<SimpleMessages>,
    ) -> Result<Response<SimpleMessages>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    async fn one_ofs_rpc(&self, request: Request<OneOfs>) -> Result<Response<OneOfs>, Status> {
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
    let server = Arc::new(SnakeCaseTypesServer::default());
    let addr: SocketAddr = test::get_test_addr();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_snake_case_types_rpc(config, server.clone()))
    })
    .bind(addr)
    .unwrap();

    tokio::spawn(http.run());

    assert_ping(
        &addr,
        "/snake-case-types/simple-messages",
        r#"{ "long_name_field": "42" }"#.into(),
    )
    .await;
    assert_ping(
        &addr,
        "/snake-case-types/oneofs",
        r#"{ "first_snake_case_value": "42" }"#.into(),
    )
    .await;
}

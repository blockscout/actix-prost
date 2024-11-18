use crate::{
    assert_ping,
    proto::serde_overrides::{
        serde_overrides_rpc_actix::route_serde_overrides_rpc,
        serde_overrides_rpc_server::SerdeOverridesRpc, CamelCaseSimpleMessages,
        CaseDependentOneOfs, SnakeCaseSimpleMessages, UnspecifiedCaseSimpleMessages,
    },
    test,
};
use actix_web::{App, HttpServer};
use std::{net::SocketAddr, sync::Arc};
use tonic::{Request, Response, Status};

#[derive(Default)]
struct SerdeOverridesServer {}

#[async_trait::async_trait]
impl SerdeOverridesRpc for SerdeOverridesServer {
    async fn snake_case_simple_messages_rpc(
        &self,
        request: Request<SnakeCaseSimpleMessages>,
    ) -> Result<Response<SnakeCaseSimpleMessages>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    async fn camel_case_simple_messages_rpc(
        &self,
        request: Request<CamelCaseSimpleMessages>,
    ) -> Result<Response<CamelCaseSimpleMessages>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    async fn unspecified_case_simple_messages_rpc(
        &self,
        request: Request<UnspecifiedCaseSimpleMessages>,
    ) -> Result<Response<UnspecifiedCaseSimpleMessages>, Status> {
        Ok(Response::new(request.into_inner()))
    }

    async fn case_dependent_one_ofs_rpc(
        &self,
        request: Request<CaseDependentOneOfs>,
    ) -> Result<Response<CaseDependentOneOfs>, Status> {
        Ok(Response::new(request.into_inner()))
    }
}

#[tokio::test]
async fn ping() {
    let server = Arc::new(SerdeOverridesServer::default());
    let addr: SocketAddr = test::get_test_addr();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_serde_overrides_rpc(config, server.clone()))
    })
    .bind(addr)
    .unwrap();

    tokio::spawn(http.run());

    assert_ping(
        &addr,
        "/serde-overrides/camel-case-simple-messages",
        r#"{ "longNameField": "42" }"#.into(),
    )
    .await;
    assert_ping(
        &addr,
        "/serde-overrides/snake-case-simple-messages",
        r#"{ "long_name_field": "42" }"#.into(),
    )
    .await;
    assert_ping(
        &addr,
        "/serde-overrides/unspecified-case-simple-messages",
        r#"{ "longNameField": "42" }"#.into(),
    )
    .await;
    assert_ping(
        &addr,
        "/serde-overrides/case-dependent-oneofs",
        r#"{ "firstCamelCaseValue": "43", "first_snake_case_value": "42", "firstUnspecifiedCaseValue": "44" }"#.into(),
    )
    .await;
}

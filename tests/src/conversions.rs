use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use crate::{
    proto::conversions::{
        conversions_rpc_actix::route_conversions_rpc, conversions_rpc_server::ConversionsRpc,
        ConfigInternal, ConfigType, ConversionsRequest, ConversionsRequestInternal,
        ConversionsResponse, ConversionsResponseInternal, MapValue, Nested,
    },
    test,
};
use actix_web::{App, HttpServer};
use convert_trait::TryConvert;
use ethers::types::Address;
use serde_json::{json, Value};
use tonic::{Request, Response, Status};

#[derive(Default)]
struct ConversionsServer {}

#[async_trait::async_trait]
impl ConversionsRpc for ConversionsServer {
    async fn convert_rpc(
        &self,
        request: Request<ConversionsRequest>,
    ) -> Result<Response<ConversionsResponse>, Status> {
        let internal_request = ConversionsRequestInternal::try_convert(request.into_inner())
            .map_err(|err| Status::invalid_argument(format!("invalid request: {}", err)))?;

        let internal_response = ConversionsResponseInternal {
            address: Address::from_low_u64_be(0),
            nested: Some(internal_request.nested),
            map_field: internal_request.map_field,
            config: None,
        };

        let response = ConversionsResponse::try_convert(internal_response)
            .map_err(|err| Status::internal(format!("internal error: {}", err)))?;

        Ok(Response::new(response))
    }
}

async fn send_post(addr: &SocketAddr, path: &str, request: Value) -> String {
    reqwest::Client::new()
        .post(format!("http://localhost:{}{}", addr.port(), path))
        .json(&request)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to send request")
        .text()
        .await
        .unwrap()
}

#[tokio::test]
async fn conversions() {
    let server = Arc::new(ConversionsServer::default());
    let addr = test::get_test_addr();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_conversions_rpc(config, server.clone()))
    })
    .bind(addr)
    .unwrap();

    tokio::spawn(http.run());

    // Invalid request
    let req = ConversionsRequest {
        map_field: HashMap::from([(
            "key".to_string(),
            MapValue {
                address: "".to_string(),
            },
        )]),
        query: "some_string".to_string(),
        addresses: vec!["".to_string()],
        nested_enum: 1,
        nested: Some(Nested {
            address: "".to_string(),
        }),
    };

    let res = send_post(&addr, "/conversions", serde_json::to_value(req).unwrap()).await;

    let res: Value = serde_json::from_str(&res).unwrap();
    assert_eq!(
        &res["message"],
        "invalid request: Invalid address: Invalid input length"
    );

    // Valid request
    let test_address = "0x000000000000000000000000000000000000dEaD".to_string();
    let req = ConversionsRequest {
        map_field: HashMap::from([(
            "key".to_string(),
            MapValue {
                address: test_address.clone(),
            },
        )]),
        query: "some_string".to_string(),
        addresses: vec![test_address.clone()],
        nested_enum: 1,
        nested: Some(Nested {
            address: test_address.clone(),
        }),
    };

    let res = send_post(&addr, "/conversions", serde_json::to_value(req).unwrap()).await;

    let res: ConversionsResponse = serde_json::from_str(&res).unwrap();
    assert_eq!(res.nested.unwrap().address, test_address);
    assert_eq!(res.map_field.get("key").unwrap().address, test_address);
}

#[test]
fn default_on_internal() {
    let config: ConfigInternal = serde_json::from_value(json!({})).unwrap();
    assert_eq!(config.r#type, ConfigType::Unspecified);
    let config: ConfigInternal = serde_json::from_value(json!({"type": "FOO"})).unwrap();
    assert_eq!(config.r#type, ConfigType::Foo);
}

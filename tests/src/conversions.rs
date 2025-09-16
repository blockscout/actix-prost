use reqwest::StatusCode;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use crate::{
    proto::conversions::{
        conversions_request::NestedEnum, conversions_rpc_actix::route_conversions_rpc,
        conversions_rpc_server::ConversionsRpc, ConfigInternal, ConfigType, ConversionsRequest,
        ConversionsRequestInternal, ConversionsResponse, ConversionsResponseInternal, MapValue,
        MapValueInternal, Nested, NestedInternal,
    },
    test,
};
use actix_web::{App, HttpServer};
use convert_trait::TryConvert;
use ethers::types::Address;
use pretty_assertions::assert_eq;
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
            response_utc_datetime: internal_request.utc_datetime,
            response_uuid: internal_request.uuid_field,
            response_decimal: internal_request.decimal_field,
        };

        let response = ConversionsResponse::try_convert(internal_response)
            .map_err(|err| Status::internal(format!("internal error: {}", err)))?;

        Ok(Response::new(response))
    }
}

async fn send_post(addr: &SocketAddr, path: &str, request: Value) -> (StatusCode, String) {
    let response = reqwest::Client::new()
        .post(format!("http://localhost:{}{}", addr.port(), path))
        .json(&request)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to send request");
    let status = response.status();
    let data = response.text().await.unwrap();
    (status, data)
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
        utc_datetime: "2023-01-01T00:00:00Z".to_string(),
        fixed_offset_datetime: "2023-01-01T00:00:00+01:00".to_string(),
        naive_datetime: "2023-01-01T00:00:00".to_string(),
        uuid_field: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        ipv4_address: "192.168.1.1".to_string(),
        ipv6_address: "2001:db8::1".to_string(),
        ip_address: "192.168.1.1".to_string(),
        path_buf: "/tmp/test".to_string(),
        duration_seconds: "1h".to_string(),
        decimal_field: "123.45".to_string(),
    };

    let (status, res) = send_post(&addr, "/conversions", serde_json::to_value(req).unwrap()).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "error: {res}");

    let res: Value = serde_json::from_str(&res).unwrap();
    assert_eq!(
        &res["message"],
        "invalid request: Invalid address: Invalid input length"
    );

    // Valid request
    let test_address = "0x000000000000000000000000000000000000dEaD";
    let test_query = "test-query";
    let test_utc_datetime = "2023-01-01T00:00:00Z";
    let test_fixed_offset_datetime = "2023-01-01T00:00:00+01:00";
    let test_naive_datetime = "2023-01-01T00:00:00";
    let test_uuid = "550e8400-e29b-41d4-a716-446655440000";
    let test_ipv4_address = "192.168.1.1";
    let test_ipv6_address = "2001:db8::1";
    let test_ip_address = "192.168.1.1";
    let test_path_buf = "/tmp/test";
    let test_duration_seconds = "60m";
    let test_decimal_field = "123.45";

    let internal_expected = ConversionsRequestInternal {
        map_field: HashMap::from([(
            "key".to_string(),
            MapValueInternal {
                address: test_address.parse().unwrap(),
            },
        )]),
        // query is overidded to Default::default() in conversions.proto
        query: Default::default(),
        addresses: vec![test_address.parse().unwrap()].into_iter().collect(),
        nested_enum: NestedEnum::NestedOk,
        nested: NestedInternal {
            address: test_address.parse().unwrap(),
        },
        utc_datetime: test_utc_datetime.parse().unwrap(),
        fixed_offset_datetime: test_fixed_offset_datetime.parse().unwrap(),
        naive_datetime: test_naive_datetime.parse().unwrap(),
        uuid_field: test_uuid.parse().unwrap(),
        ipv4_address: test_ipv4_address.parse().unwrap(),
        ipv6_address: test_ipv6_address.parse().unwrap(),
        ip_address: test_ip_address.parse().unwrap(),
        path_buf: test_path_buf.parse().unwrap(),
        duration_seconds: Duration::from_secs(3600),
        decimal_field: test_decimal_field.parse().unwrap(),
        field1: None,
        field2: None,
    };

    let req = ConversionsRequest {
        map_field: HashMap::from([(
            "key".to_string(),
            MapValue {
                address: test_address.to_string(),
            },
        )]),
        query: test_query.to_string(),
        addresses: vec![test_address.to_string()],
        nested_enum: NestedEnum::NestedOk as i32,
        nested: Some(Nested {
            address: test_address.to_string(),
        }),
        utc_datetime: test_utc_datetime.to_string(),
        fixed_offset_datetime: test_fixed_offset_datetime.to_string(),
        naive_datetime: test_naive_datetime.to_string(),
        uuid_field: test_uuid.to_string(),
        ipv4_address: test_ipv4_address.to_string(),
        ipv6_address: test_ipv6_address.to_string(),
        ip_address: test_ip_address.to_string(),
        path_buf: test_path_buf.to_string(),
        duration_seconds: test_duration_seconds.to_string(),
        decimal_field: test_decimal_field.to_string(),
    };

    let req_internal = ConversionsRequestInternal::try_convert(req.clone()).unwrap();
    assert_eq!(req_internal, internal_expected);

    let (status, res) = send_post(&addr, "/conversions", serde_json::to_value(req).unwrap()).await;
    assert_eq!(status, StatusCode::OK, "error: {res}");
    let res: ConversionsResponse = serde_json::from_str(&res).unwrap();
    assert_eq!(res.nested.unwrap().address, test_address);
}

#[test]
fn default_on_internal() {
    let config: ConfigInternal = serde_json::from_value(json!({})).unwrap();
    assert_eq!(config.r#type, ConfigType::Unspecified);
    let config: ConfigInternal = serde_json::from_value(json!({"type": null})).unwrap();
    assert_eq!(config.r#type, ConfigType::Unspecified);
    let config: ConfigInternal = serde_json::from_value(json!({"type": "FOO"})).unwrap();
    assert_eq!(config.r#type, ConfigType::Foo);
}

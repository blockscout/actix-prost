use crate::{
    proto::errors::{
        errors_rpc_actix::route_errors_rpc, errors_rpc_server::ErrorsRpc, ErrorRequest,
        ErrorResponse,
    },
    test,
};
use actix_http::StatusCode;
use actix_prost::{http_compatibility::to_actix_status, Error};
use actix_web::{App, HttpServer};
use pretty_assertions::assert_eq;
use serde::de::DeserializeOwned;
use std::{net::SocketAddr, sync::Arc};
use tonic::{Code, Request, Response, Status};

#[derive(Default)]
struct ErrorsServer {}

#[async_trait::async_trait]
impl ErrorsRpc for ErrorsServer {
    async fn error(
        &self,
        request: Request<ErrorRequest>,
    ) -> Result<Response<ErrorResponse>, Status> {
        let request = request.into_inner();
        let code = Code::from(request.code);
        Err(Status::new(code, request.message))
    }
}

async fn send_post<T: DeserializeOwned>(
    addr: &SocketAddr,
    path: &str,
    body: String,
) -> (T, StatusCode) {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://localhost:{}{}", addr.port(), path))
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();
    let status = response.status();
    let data = response.text().await.unwrap();
    (
        serde_json::from_str(&data)
            .unwrap_or_else(|_| panic!("could not parse json, got: {}", data)),
        to_actix_status(status),
    )
}

async fn send_code(addr: &SocketAddr, code: Code) {
    assert_eq!(
        send_post::<Error>(
            addr,
            &format!("/errors/{}?query=something", i32::from(code)),
            format!(r#"{{"message":"status {}"}}"#, code),
        )
        .await,
        (
            Error {
                code,
                message: format!("status {}", code)
            },
            to_actix_status(Error::map_tonic_code(code))
        )
    );
}

#[tokio::test]
async fn errors() {
    let server = Arc::new(ErrorsServer::default());
    let addr = test::get_test_addr();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_errors_rpc(config, server.clone()))
    })
    .bind(addr)
    .unwrap();

    tokio::spawn(http.run());

    for i in 0..17 {
        send_code(&addr, Code::from(i)).await;
    }

    assert_eq!(
        send_post::<Error>(
            &addr,
            "/errors/hi?query=something",
            r#"{"message":"path error"}"#.into(),
        )
        .await,
        (
            Error {
                code: Code::InvalidArgument,
                message: "can not parse \"hi\" to a i32".into()
            },
            StatusCode::BAD_REQUEST
        )
    );
    assert_eq!(
        send_post::<Error>(&addr, "/errors/0", r#"{"message":"query error"}"#.into()).await,
        (
            Error {
                code: Code::InvalidArgument,
                message: "Query deserialize error: missing field `query`".into()
            },
            StatusCode::BAD_REQUEST
        )
    );
    assert_eq!(
        send_post::<Error>(&addr, "/errors/0?query=something", r#"{}"#.into()).await,
        (
            Error {
                code: Code::InvalidArgument,
                message: "Json deserialize error: missing field `message` at line 1 column 2"
                    .into()
            },
            StatusCode::BAD_REQUEST
        )
    );
}

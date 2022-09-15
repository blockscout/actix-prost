use crate::proto::rest::{rest_rpc_actix::route_rest_rpc, rest_rpc_server::RestRpc, Get, Post};
use actix_web::{App, HttpServer};
use serde::de::DeserializeOwned;
use std::{net::SocketAddr, sync::Arc};
use tonic::{Request, Response, Status};

#[derive(Default)]
struct RestServer {}

#[async_trait::async_trait]
impl RestRpc for RestServer {
    async fn get_rpc(&self, request: Request<Get>) -> Result<Response<Get>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn get_query_rpc(&self, request: Request<Get>) -> Result<Response<Get>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn post_rpc(&self, request: Request<Post>) -> Result<Response<Post>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn post_query_rpc(&self, request: Request<Post>) -> Result<Response<Post>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn post_no_path_rpc(&self, request: Request<Post>) -> Result<Response<Post>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn post_get_rpc(&self, request: Request<Post>) -> Result<Response<Get>, Status> {
        let request = request.into_inner();
        Ok(Response::new(Get {
            foo: request.foo,
            bar: request.bar,
        }))
    }
}

async fn send_get(path: &str) -> Get {
    reqwest::get("http://localhost:8042".to_string() + path)
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn send_post<T: DeserializeOwned>(path: &str, body: String) -> T {
    let client = reqwest::Client::new();
    client
        .post("http://localhost:8042".to_string() + path)
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[tokio::test]
async fn test_ok() {
    let server = Arc::new(RestServer::default());
    let httpaddr: SocketAddr = "[::]:8042".parse().unwrap();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_rest_rpc(config, server.clone()))
    })
    .bind(httpaddr)
    .unwrap();

    tokio::spawn(http.run());

    let get = Get {
        foo: "hello".into(),
        bar: 234,
    };
    let post = Post {
        foo: "world".into(),
        bar: 345,
        baz: 123.563,
    };

    assert_eq!(
        send_get(&format!("/rest/get/{}/{}", get.foo, get.bar)).await,
        get
    );
    assert_eq!(
        send_get(&format!("/rest/get/{}?bar={}", get.foo, get.bar)).await,
        get
    );

    assert_eq!(
        send_post::<Post>(
            &format!("/rest/post/{}/{}", post.foo, post.bar),
            format!(r#"{{"baz":{}}}"#, post.baz),
        )
        .await,
        post
    );
    assert_eq!(
        send_post::<Post>(
            &format!("/rest/post/{}?bar={}", post.foo, post.bar),
            format!(r#"{{"baz":{}}}"#, post.baz),
        )
        .await,
        post
    );
    assert_eq!(
        send_post::<Post>(
            &format!("/rest/post"),
            format!(
                r#"{{"foo":"{}","bar":{},"baz":{}}}"#,
                post.foo, post.bar, post.baz
            ),
        )
        .await,
        post
    );

    assert_eq!(
        send_post::<Get>(
            &format!("/rest/post_get"),
            format!(
                r#"{{"foo":"{}","bar":{},"baz":{}}}"#,
                post.foo, post.bar, post.baz
            ),
        )
        .await,
        Get {
            foo: post.foo,
            bar: post.bar
        }
    );
}

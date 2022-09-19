use crate::proto::rest::{
    rest_rpc_actix::route_rest_rpc, rest_rpc_server::RestRpc, simple_rpc_actix::route_simple_rpc,
    simple_rpc_server::SimpleRpc, Get, Post,
};
use actix_web::{App, HttpServer};
use pretty_assertions::assert_eq;
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

async fn send_get(addr: &SocketAddr, path: &str) -> Get {
    reqwest::get(format!("http://localhost:{}{}", addr.port(), path))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn send_post<T: DeserializeOwned>(addr: &SocketAddr, path: &str, body: String) -> T {
    let client = reqwest::Client::new();
    client
        .post(format!("http://localhost:{}{}", addr.port(), path))
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
async fn ping() {
    let server = Arc::new(RestServer::default());
    let addr: SocketAddr = "[::]:8042".parse().unwrap();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_rest_rpc(config, server.clone()))
    })
    .bind(addr)
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
        send_get(&addr, &format!("/rest/get/{}/{}", get.foo, get.bar)).await,
        get
    );
    assert_eq!(
        send_get(&addr, &format!("/rest/get/{}?bar={}", get.foo, get.bar)).await,
        get
    );

    assert_eq!(
        send_post::<Post>(
            &addr,
            &format!("/rest/post/{}/{}", post.foo, post.bar),
            format!(r#"{{"baz":{}}}"#, post.baz),
        )
        .await,
        post
    );
    assert_eq!(
        send_post::<Post>(
            &addr,
            &format!("/rest/post/{}?bar={}", post.foo, post.bar),
            format!(r#"{{"baz":{}}}"#, post.baz),
        )
        .await,
        post
    );
    assert_eq!(
        send_post::<Post>(
            &addr,
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
            &addr,
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

#[derive(Default)]
struct HeaderServer {}

#[async_trait::async_trait]
impl SimpleRpc for HeaderServer {
    async fn post_rpc(&self, request: Request<Post>) -> Result<Response<Post>, Status> {
        let mut meta = request
            .metadata()
            .iter()
            .map(|kv| format!("{:?}", kv))
            .collect::<Vec<_>>();
        meta.sort();
        let meta = meta.join(",");
        Ok(Response::new(Post {
            foo: meta,
            bar: request.get_ref().bar,
            baz: request.get_ref().baz,
        }))
    }
}

#[tokio::test]
async fn headers() {
    let server = Arc::new(HeaderServer::default());
    let addr: SocketAddr = "[::]:8043".parse().unwrap();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_simple_rpc(config, server.clone()))
    })
    .bind(addr)
    .unwrap();

    tokio::spawn(http.run());

    let post = Post {
        foo: "world".into(),
        bar: 345,
        baz: 123.563,
    };

    assert_eq!(
        send_post::<Post>(
            &addr,
            &format!("/rest/post/{}?bar={}", post.foo, post.bar),
            format!(r#"{{"baz":{}}}"#, post.baz),
        )
        .await,
        Post {
            foo: r#"Ascii("accept", "*/*"),Ascii("content-length", "15"),Ascii("content-type", "application/json"),Ascii("host", "localhost:8043")"#.into(),
            bar: post.bar,
            baz: post.baz,
        }
    );
}

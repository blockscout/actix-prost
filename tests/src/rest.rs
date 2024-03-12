use crate::{
    proto::{
        rest::{rest_rpc_actix::route_rest_rpc, rest_rpc_server::RestRpc, Get, Post},
        simple::{
            simple_rpc_actix::route_simple_rpc, simple_rpc_server::SimpleRpc, Post as SimplePost,
        },
    },
    test,
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
    async fn get_response_rpc(&self, request: Request<Get>) -> Result<Response<Get>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn post_response_rpc(&self, request: Request<Post>) -> Result<Response<Post>, Status> {
        Ok(Response::new(request.into_inner()))
    }
    async fn post_response_get_rpc(&self, request: Request<Post>) -> Result<Response<Get>, Status> {
        let request = request.into_inner();
        Ok(Response::new(Get {
            foo: request.foo,
            bar: request.bar,
        }))
    }
}

async fn send_get<T: DeserializeOwned>(addr: &SocketAddr, path: &str) -> T {
    reqwest::get(format!("http://localhost:{}{}", addr.port(), path))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn send_post<T: DeserializeOwned>(addr: &SocketAddr, path: &str, body: String) -> T {
    let client = reqwest::Client::new();
    let data = client
        .post(format!("http://localhost:{}{}", addr.port(), path))
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| panic!("could not parse json, got: {}", data))
}

#[tokio::test]
async fn request() {
    let server = Arc::new(RestServer::default());
    let addr = test::get_test_addr();
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
        long_name: 123.563,
    };

    assert_eq!(
        send_get::<Get>(&addr, &format!("/rest/get/{}/{}", get.foo, get.bar)).await,
        get
    );
    assert_eq!(
        send_get::<Get>(&addr, &format!("/rest/get/{}?bar={}", get.foo, get.bar)).await,
        get
    );

    assert_eq!(
        send_post::<Post>(
            &addr,
            &format!("/rest/post/{}/{}", post.foo, post.bar),
            format!(r#"{{"longName":{}}}"#, post.long_name),
        )
        .await,
        post
    );
    assert_eq!(
        send_post::<Post>(
            &addr,
            &format!("/rest/post/{}?bar={}", post.long_name, post.bar),
            format!(r#"{{"foo":"{}"}}"#, post.foo),
        )
        .await,
        post
    );
    assert_eq!(
        send_post::<Post>(
            &addr,
            "/rest/post",
            format!(
                r#"{{"foo":"{}","bar":"{}","longName":{}}}"#,
                post.foo, post.bar, post.long_name
            ),
        )
        .await,
        post
    );

    assert_eq!(
        send_post::<Get>(
            &addr,
            "/rest/post_get",
            format!(
                r#"{{"foo":"{}","bar":"{}","longName":{}}}"#,
                post.foo, post.bar, post.long_name
            ),
        )
        .await,
        Get {
            foo: post.foo,
            bar: post.bar
        }
    );
}

#[tokio::test]
async fn response() {
    let server = Arc::new(RestServer::default());
    let addr = test::get_test_addr();
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
        long_name: 123.563,
    };

    assert_eq!(
        send_get::<String>(
            &addr,
            &format!("/rest/response/get/{}/{}", get.foo, get.bar)
        )
        .await,
        get.foo.to_owned()
    );

    assert_eq!(
        send_post::<i64>(
            &addr,
            "/rest/response/post",
            format!(
                r#"{{"foo":"{}","bar":"{}","longName":{}}}"#,
                post.foo, post.bar, post.long_name
            ),
        )
        .await,
        post.bar
    );

    assert_eq!(
        send_post::<String>(
            &addr,
            "/rest/response/post_get",
            format!(
                r#"{{"foo":"{}","bar":"{}","longName":{}}}"#,
                post.foo, post.bar, post.long_name
            ),
        )
        .await,
        post.foo
    );
}
#[derive(Default)]
struct HeaderServer {}

#[async_trait::async_trait]
impl SimpleRpc for HeaderServer {
    async fn post_rpc(&self, request: Request<SimplePost>) -> Result<Response<SimplePost>, Status> {
        let mut meta = request
            .metadata()
            .iter()
            .map(|kv| format!("{:?}", kv))
            .collect::<Vec<_>>();
        meta.sort();
        let meta = meta.join(",");
        Ok(Response::new(SimplePost {
            foo: meta,
            bar: request.get_ref().bar,
            long_name: request.get_ref().long_name,
        }))
    }
}

#[tokio::test]
async fn headers() {
    let server = Arc::new(HeaderServer::default());
    let addr = test::get_test_addr();
    let http = HttpServer::new(move || {
        App::new().configure(|config| route_simple_rpc(config, server.clone()))
    })
    .bind(addr)
    .unwrap();

    tokio::spawn(http.run());

    let post = Post {
        foo: "world".into(),
        bar: 345,
        long_name: 123.563,
    };

    assert_eq!(
        send_post::<Post>(
            &addr,
            &format!("/rest/post/{}?bar={}", post.foo, post.bar),
            format!(r#"{{"longName":{}}}"#, post.long_name),
        )
        .await,
        Post {
            foo: format!(
                r#"Ascii("accept", "*/*"),Ascii("content-length", "20"),Ascii("content-type", "application/json"),Ascii("host", "localhost:{}")"#,
                addr.port()
            ),
            bar: post.bar,
            long_name: post.long_name,
        }
    );
}

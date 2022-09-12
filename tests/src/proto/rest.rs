#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get {
    #[prost(string, tag="1")]
    pub foo: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub bar: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {
    #[prost(string, tag="1")]
    pub foo: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub bar: i64,
    #[prost(double, tag="3")]
    pub baz: f64,
}
pub mod rest_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::rest_rpc_server::RestRpc;
    use tonic::IntoRequest;
    use actix_web::{
        web::{self, Json, ServiceConfig, Data, Path},
        error::Error,
    };
    use std::sync::Arc;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetRPCPath {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub bar: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostRPCPath {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub bar: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostWildcardRPCPath {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostRPCBody {
        #[prost(double, tag = "3")]
        pub baz: f64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostNoPathRPCBody {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub bar: i64,
        #[prost(double, tag = "3")]
        pub baz: f64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostWildcardRPCBody {
        #[prost(int64, tag = "2")]
        pub bar: i64,
        #[prost(double, tag = "3")]
        pub baz: f64,
    }
    async fn call_get_rpc(
        service: Data<dyn RestRpc>,
        path: Path<GetRPCPath>,
    ) -> Result<Json<Get>, Error> {
        let path = path.into_inner();
        let request = Get {
            foo: path.foo,
            bar: path.bar,
        };
        Ok(
            Json(
                service
                    .get_rpc(request.into_request())
                    .await
                    .map_err(actix_web::error::ErrorNotImplemented)?
                    .into_inner(),
            ),
        )
    }
    async fn call_post_rpc(
        service: Data<dyn RestRpc>,
        path: Path<PostRPCPath>,
        body: Json<PostRPCBody>,
    ) -> Result<Json<Post>, Error> {
        let path = path.into_inner();
        let body = body.into_inner();
        let request = Post {
            foo: path.foo,
            bar: path.bar,
            baz: body.baz,
        };
        Ok(
            Json(
                service
                    .post_rpc(request.into_request())
                    .await
                    .map_err(actix_web::error::ErrorNotImplemented)?
                    .into_inner(),
            ),
        )
    }
    async fn call_post_no_path_rpc(
        service: Data<dyn RestRpc>,
        body: Json<PostNoPathRPCBody>,
    ) -> Result<Json<Post>, Error> {
        let body = body.into_inner();
        let request = Post {
            foo: body.foo,
            bar: body.bar,
            baz: body.baz,
        };
        Ok(
            Json(
                service
                    .post_no_path_rpc(request.into_request())
                    .await
                    .map_err(actix_web::error::ErrorNotImplemented)?
                    .into_inner(),
            ),
        )
    }
    async fn call_post_wildcard_rpc(
        service: Data<dyn RestRpc>,
        path: Path<PostWildcardRPCPath>,
        body: Json<PostWildcardRPCBody>,
    ) -> Result<Json<Post>, Error> {
        let path = path.into_inner();
        let body = body.into_inner();
        let request = Post {
            foo: path.foo,
            bar: body.bar,
            baz: body.baz,
        };
        Ok(
            Json(
                service
                    .post_wildcard_rpc(request.into_request())
                    .await
                    .map_err(actix_web::error::ErrorNotImplemented)?
                    .into_inner(),
            ),
        )
    }
    pub fn route_rest_rpc(config: &mut ServiceConfig, service: Arc<dyn RestRpc>) {
        config.app_data(Data::from(service));
        config.route("/rest/get/{foo}/{bar}", web::get().to(call_get_rpc));
        config.route("/rest/post/{foo}/{bar}", web::post().to(call_post_rpc));
        config.route("/rest/post", web::post().to(call_post_no_path_rpc));
        config.route("/rest/post/{foo}", web::post().to(call_post_wildcard_rpc));
    }
}
/// Generated client implementations.
pub mod rest_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RestRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RestRpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RestRpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RestRpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            RestRpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Get>,
        ) -> Result<tonic::Response<super::Get>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rest.RestRPC/GetRPC");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rest.RestRPC/PostRPC");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_no_path_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rest.RestRPC/PostNoPathRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_wildcard_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/rest.RestRPC/PostWildcardRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod rest_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with RestRpcServer.
    #[async_trait]
    pub trait RestRpc: Send + Sync + 'static {
        async fn get_rpc(
            &self,
            request: tonic::Request<super::Get>,
        ) -> Result<tonic::Response<super::Get>, tonic::Status>;
        async fn post_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn post_no_path_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn post_wildcard_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RestRpcServer<T: RestRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RestRpc> RestRpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RestRpcServer<T>
    where
        T: RestRpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rest.RestRPC/GetRPC" => {
                    #[allow(non_camel_case_types)]
                    struct GetRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Get>
                    for GetRPCSvc<T> {
                        type Response = super::Get;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Get>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRPCSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rest.RestRPC/PostRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Post>
                    for PostRPCSvc<T> {
                        type Response = super::Post;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Post>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).post_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostRPCSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rest.RestRPC/PostNoPathRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostNoPathRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Post>
                    for PostNoPathRPCSvc<T> {
                        type Response = super::Post;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Post>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_no_path_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostNoPathRPCSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rest.RestRPC/PostWildcardRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostWildcardRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Post>
                    for PostWildcardRPCSvc<T> {
                        type Response = super::Post;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Post>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).post_wildcard_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostWildcardRPCSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: RestRpc> Clone for RestRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RestRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RestRpc> tonic::server::NamedService for RestRpcServer<T> {
        const NAME: &'static str = "rest.RestRPC";
    }
}

#[actix_prost_macros::serde]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get {
    #[prost(string, tag="1")]
    pub foo: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub bar: i64,
}
#[actix_prost_macros::serde]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {
    #[prost(string, tag="1")]
    pub foo: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub bar: i64,
    #[prost(double, tag="3")]
    pub long_name: f64,
}
pub mod rest_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::rest_rpc_server::RestRpc;
    use std::sync::Arc;
    type GetRPCPath = Get;
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetQueryRPCPath {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetQueryRPCQuery {
        #[prost(int64, tag = "2")]
        pub bar: i64,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostRPCPath {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub bar: i64,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostRPCJson {
        #[prost(double, tag = "3")]
        pub long_name: f64,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostQueryRPCPath {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostQueryRPCQuery {
        #[prost(int64, tag = "2")]
        pub bar: i64,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostQueryRPCJson {
        #[prost(double, tag = "3")]
        pub long_name: f64,
    }
    type PostNoPathRPCJson = Post;
    type PostGetRPCJson = Post;
    type GetResponseRPCPath = Get;
    type PostResponseRPCJson = Post;
    type PostResponseGetRPCJson = Post;
    async fn call_get_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<Get>, ::actix_web::Error> {
        let path = <::actix_web::web::Path::<
            GetRPCPath,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let request = Get {
            foo: path.foo,
            bar: path.bar,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .get_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_query_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<Get>, ::actix_web::Error> {
        let path = <::actix_web::web::Path::<
            GetQueryRPCPath,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let query = <::actix_web::web::Query::<
            GetQueryRPCQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let request = Get {
            foo: path.foo,
            bar: query.bar,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .get_query_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_post_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Post>, ::actix_web::Error> {
        let mut payload = payload.into_inner();
        let path = <::actix_web::web::Path::<
            PostRPCPath,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let json = <::actix_web::web::Json::<
            PostRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = Post {
            foo: path.foo,
            bar: path.bar,
            long_name: json.long_name,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .post_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_post_query_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Post>, ::actix_web::Error> {
        let mut payload = payload.into_inner();
        let path = <::actix_web::web::Path::<
            PostQueryRPCPath,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let query = <::actix_web::web::Query::<
            PostQueryRPCQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let json = <::actix_web::web::Json::<
            PostQueryRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = Post {
            foo: path.foo,
            bar: query.bar,
            long_name: json.long_name,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .post_query_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_post_no_path_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Post>, ::actix_web::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            PostNoPathRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = Post {
            foo: json.foo,
            bar: json.bar,
            long_name: json.long_name,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .post_no_path_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_post_get_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Get>, ::actix_web::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            PostGetRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = Post {
            foo: json.foo,
            bar: json.bar,
            long_name: json.long_name,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .post_get_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_get_response_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<
        ::actix_web::web::Json<::prost::alloc::string::String>,
        ::actix_web::Error,
    > {
        let path = <::actix_web::web::Path::<
            GetResponseRPCPath,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let request = Get {
            foo: path.foo,
            bar: path.bar,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .get_response_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        let response = response.foo;
        Ok(::actix_web::web::Json(response))
    }
    async fn call_post_response_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<i64>, ::actix_web::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            PostResponseRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = Post {
            foo: json.foo,
            bar: json.bar,
            long_name: json.long_name,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .post_response_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        let response = response.bar;
        Ok(::actix_web::web::Json(response))
    }
    async fn call_post_response_get_rpc(
        service: ::actix_web::web::Data<dyn RestRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<::prost::alloc::string::String>,
        ::actix_web::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            PostResponseGetRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = Post {
            foo: json.foo,
            bar: json.bar,
            long_name: json.long_name,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .post_response_get_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        let response = response.foo;
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_rest_rpc(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn RestRpc + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route("/rest/get/{foo}/{bar}", ::actix_web::web::get().to(call_get_rpc));
        config.route("/rest/get/{foo}", ::actix_web::web::get().to(call_get_query_rpc));
        config
            .route("/rest/post/{foo}/{bar}", ::actix_web::web::post().to(call_post_rpc));
        config
            .route("/rest/post/{foo}", ::actix_web::web::post().to(call_post_query_rpc));
        config.route("/rest/post", ::actix_web::web::post().to(call_post_no_path_rpc));
        config.route("/rest/post_get", ::actix_web::web::post().to(call_post_get_rpc));
        config
            .route(
                "/rest/response/get/{foo}/{bar}",
                ::actix_web::web::get().to(call_get_response_rpc),
            );
        config
            .route(
                "/rest/response/post",
                ::actix_web::web::post().to(call_post_response_rpc),
            );
        config
            .route(
                "/rest/response/post_get",
                ::actix_web::web::post().to(call_post_response_get_rpc),
            );
    }
}
pub mod simple_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::simple_rpc_server::SimpleRpc;
    use std::sync::Arc;
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostRPCPath {
        #[prost(string, tag = "1")]
        pub foo: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostRPCQuery {
        #[prost(int64, tag = "2")]
        pub bar: i64,
    }
    #[actix_prost_macros::serde]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PostRPCJson {
        #[prost(double, tag = "3")]
        pub long_name: f64,
    }
    async fn call_post_rpc(
        service: ::actix_web::web::Data<dyn SimpleRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Post>, ::actix_web::Error> {
        let mut payload = payload.into_inner();
        let path = <::actix_web::web::Path::<
            PostRPCPath,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let query = <::actix_web::web::Query::<
            PostRPCQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await?
            .into_inner();
        let json = <::actix_web::web::Json::<
            PostRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = Post {
            foo: path.foo,
            bar: query.bar,
            long_name: json.long_name,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .post_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_simple_rpc(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn SimpleRpc + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route("/rest/post/{foo}", ::actix_web::web::post().to(call_post_rpc));
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
        pub async fn get_query_rpc(
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
            let path = http::uri::PathAndQuery::from_static("/rest.RestRPC/GetQueryRPC");
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
        pub async fn post_query_rpc(
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
                "/rest.RestRPC/PostQueryRPC",
            );
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
        pub async fn post_get_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Post>,
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
            let path = http::uri::PathAndQuery::from_static("/rest.RestRPC/PostGetRPC");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_response_rpc(
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
            let path = http::uri::PathAndQuery::from_static(
                "/rest.RestRPC/GetResponseRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_response_rpc(
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
                "/rest.RestRPC/PostResponseRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn post_response_get_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Post>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/rest.RestRPC/PostResponseGetRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod simple_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SimpleRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SimpleRpcClient<tonic::transport::Channel> {
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
    impl<T> SimpleRpcClient<T>
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
        ) -> SimpleRpcClient<InterceptedService<T, F>>
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
            SimpleRpcClient::new(InterceptedService::new(inner, interceptor))
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
            let path = http::uri::PathAndQuery::from_static("/rest.SimpleRPC/PostRPC");
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
        async fn get_query_rpc(
            &self,
            request: tonic::Request<super::Get>,
        ) -> Result<tonic::Response<super::Get>, tonic::Status>;
        async fn post_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn post_query_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn post_no_path_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn post_get_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Get>, tonic::Status>;
        async fn get_response_rpc(
            &self,
            request: tonic::Request<super::Get>,
        ) -> Result<tonic::Response<super::Get>, tonic::Status>;
        async fn post_response_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
        async fn post_response_get_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Get>, tonic::Status>;
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
                "/rest.RestRPC/GetQueryRPC" => {
                    #[allow(non_camel_case_types)]
                    struct GetQueryRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Get>
                    for GetQueryRPCSvc<T> {
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
                            let fut = async move {
                                (*inner).get_query_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetQueryRPCSvc(inner);
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
                "/rest.RestRPC/PostQueryRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostQueryRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Post>
                    for PostQueryRPCSvc<T> {
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
                                (*inner).post_query_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostQueryRPCSvc(inner);
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
                "/rest.RestRPC/PostGetRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostGetRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Post>
                    for PostGetRPCSvc<T> {
                        type Response = super::Get;
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
                                (*inner).post_get_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostGetRPCSvc(inner);
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
                "/rest.RestRPC/GetResponseRPC" => {
                    #[allow(non_camel_case_types)]
                    struct GetResponseRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Get>
                    for GetResponseRPCSvc<T> {
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
                            let fut = async move {
                                (*inner).get_response_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetResponseRPCSvc(inner);
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
                "/rest.RestRPC/PostResponseRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostResponseRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Post>
                    for PostResponseRPCSvc<T> {
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
                                (*inner).post_response_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostResponseRPCSvc(inner);
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
                "/rest.RestRPC/PostResponseGetRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostResponseGetRPCSvc<T: RestRpc>(pub Arc<T>);
                    impl<T: RestRpc> tonic::server::UnaryService<super::Post>
                    for PostResponseGetRPCSvc<T> {
                        type Response = super::Get;
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
                                (*inner).post_response_get_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PostResponseGetRPCSvc(inner);
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
/// Generated server implementations.
pub mod simple_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SimpleRpcServer.
    #[async_trait]
    pub trait SimpleRpc: Send + Sync + 'static {
        async fn post_rpc(
            &self,
            request: tonic::Request<super::Post>,
        ) -> Result<tonic::Response<super::Post>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SimpleRpcServer<T: SimpleRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SimpleRpc> SimpleRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SimpleRpcServer<T>
    where
        T: SimpleRpc,
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
                "/rest.SimpleRPC/PostRPC" => {
                    #[allow(non_camel_case_types)]
                    struct PostRPCSvc<T: SimpleRpc>(pub Arc<T>);
                    impl<T: SimpleRpc> tonic::server::UnaryService<super::Post>
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
    impl<T: SimpleRpc> Clone for SimpleRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SimpleRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SimpleRpc> tonic::server::NamedService for SimpleRpcServer<T> {
        const NAME: &'static str = "rest.SimpleRPC";
    }
}

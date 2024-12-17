#[actix_prost_macros::serde]
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleMessages {
    #[prost(int64, tag = "1")]
    pub long_name_field: i64,
}
#[actix_prost_macros::serde]
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneOfs {
    #[prost(oneof = "one_ofs::SnakeCaseValues", tags = "1, 2")]
    pub snake_case_values: ::core::option::Option<one_ofs::SnakeCaseValues>,
}
/// Nested message and enum types in `OneOfs`.
pub mod one_ofs {
    #[actix_prost_macros::serde]
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SnakeCaseValues {
        #[prost(int64, tag = "1")]
        FirstSnakeCaseValue(i64),
        #[prost(int64, tag = "2")]
        SecondSnakeCaseValue(i64),
    }
}
pub mod snake_case_types_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::snake_case_types_rpc_server::SnakeCaseTypesRpc;
    use std::sync::Arc;
    #[actix_prost_macros::serde]
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SimpleMessagesRPCJson {
        #[prost(int64, tag = "1")]
        pub long_name_field: i64,
    }
    #[actix_prost_macros::serde]
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OneOfsRPCJson {
        #[prost(oneof = "one_ofs::SnakeCaseValues", tags = "1, 2")]
        pub snake_case_values: ::core::option::Option<one_ofs::SnakeCaseValues>,
    }
    async fn call_simple_messages_rpc(
        service: ::actix_web::web::Data<dyn SnakeCaseTypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<SimpleMessages>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SimpleMessagesRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SimpleMessages {
            long_name_field: json.long_name_field,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.simple_messages_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_one_ofs_rpc(
        service: ::actix_web::web::Data<dyn SnakeCaseTypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<OneOfs>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            OneOfsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = OneOfs {
            snake_case_values: json.snake_case_values,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.one_ofs_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_snake_case_types_rpc(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn SnakeCaseTypesRpc + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config
            .route(
                "/snake-case-types/simple-messages",
                ::actix_web::web::post().to(call_simple_messages_rpc),
            );
        config
            .route(
                "/snake-case-types/oneofs",
                ::actix_web::web::post().to(call_one_ofs_rpc),
            );
    }
}
#[derive(Clone, Debug)]
pub struct SimpleMessagesInternal {
    pub long_name_field: i64,
}
impl convert_trait::TryConvert<SimpleMessages> for SimpleMessagesInternal {
    fn try_convert(from: SimpleMessages) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
impl convert_trait::TryConvert<SimpleMessagesInternal> for SimpleMessages {
    fn try_convert(from: SimpleMessagesInternal) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
#[derive(Clone, Debug)]
pub struct OneOfsInternal {
    pub snake_case_values: ::core::option::Option<one_ofs::SnakeCaseValues>,
}
impl convert_trait::TryConvert<OneOfs> for OneOfsInternal {
    fn try_convert(from: OneOfs) -> Result<Self, String> {
        Ok(Self {
            snake_case_values: from.snake_case_values,
        })
    }
}
impl convert_trait::TryConvert<OneOfsInternal> for OneOfs {
    fn try_convert(from: OneOfsInternal) -> Result<Self, String> {
        Ok(Self {
            snake_case_values: from.snake_case_values,
        })
    }
}
/// Generated client implementations.
pub mod snake_case_types_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SnakeCaseTypesRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SnakeCaseTypesRpcClient<tonic::transport::Channel> {
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
    impl<T> SnakeCaseTypesRpcClient<T>
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
        ) -> SnakeCaseTypesRpcClient<InterceptedService<T, F>>
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
            SnakeCaseTypesRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn simple_messages_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleMessages>,
        ) -> Result<tonic::Response<super::SimpleMessages>, tonic::Status> {
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
                "/snake_case_types.SnakeCaseTypesRPC/SimpleMessagesRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn one_ofs_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::OneOfs>,
        ) -> Result<tonic::Response<super::OneOfs>, tonic::Status> {
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
                "/snake_case_types.SnakeCaseTypesRPC/OneOfsRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod snake_case_types_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SnakeCaseTypesRpcServer.
    #[async_trait]
    pub trait SnakeCaseTypesRpc: Send + Sync + 'static {
        async fn simple_messages_rpc(
            &self,
            request: tonic::Request<super::SimpleMessages>,
        ) -> Result<tonic::Response<super::SimpleMessages>, tonic::Status>;
        async fn one_ofs_rpc(
            &self,
            request: tonic::Request<super::OneOfs>,
        ) -> Result<tonic::Response<super::OneOfs>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SnakeCaseTypesRpcServer<T: SnakeCaseTypesRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SnakeCaseTypesRpc> SnakeCaseTypesRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SnakeCaseTypesRpcServer<T>
    where
        T: SnakeCaseTypesRpc,
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
                "/snake_case_types.SnakeCaseTypesRPC/SimpleMessagesRPC" => {
                    #[allow(non_camel_case_types)]
                    struct SimpleMessagesRPCSvc<T: SnakeCaseTypesRpc>(pub Arc<T>);
                    impl<
                        T: SnakeCaseTypesRpc,
                    > tonic::server::UnaryService<super::SimpleMessages>
                    for SimpleMessagesRPCSvc<T> {
                        type Response = super::SimpleMessages;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleMessages>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).simple_messages_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SimpleMessagesRPCSvc(inner);
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
                "/snake_case_types.SnakeCaseTypesRPC/OneOfsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct OneOfsRPCSvc<T: SnakeCaseTypesRpc>(pub Arc<T>);
                    impl<T: SnakeCaseTypesRpc> tonic::server::UnaryService<super::OneOfs>
                    for OneOfsRPCSvc<T> {
                        type Response = super::OneOfs;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OneOfs>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).one_ofs_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OneOfsRPCSvc(inner);
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
    impl<T: SnakeCaseTypesRpc> Clone for SnakeCaseTypesRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SnakeCaseTypesRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SnakeCaseTypesRpc> tonic::server::NamedService
    for SnakeCaseTypesRpcServer<T> {
        const NAME: &'static str = "snake_case_types.SnakeCaseTypesRPC";
    }
}

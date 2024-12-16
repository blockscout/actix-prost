#[actix_prost_macros::serde]
#[actix_prost_macros::serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CamelCaseSimpleMessages {
    #[prost(int64, tag = "1")]
    pub long_name_field: i64,
}
#[actix_prost_macros::serde]
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnakeCaseSimpleMessages {
    #[prost(int64, tag = "1")]
    pub long_name_field: i64,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnspecifiedCaseSimpleMessages {
    #[prost(int64, tag = "1")]
    pub long_name_field: i64,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseDependentOneOfs {
    #[prost(oneof = "case_dependent_one_ofs::CamelCaseValues", tags = "1, 2")]
    pub camel_case_values: ::core::option::Option<
        case_dependent_one_ofs::CamelCaseValues,
    >,
    #[prost(oneof = "case_dependent_one_ofs::SnakeCaseValues", tags = "3, 4")]
    pub snake_case_values: ::core::option::Option<
        case_dependent_one_ofs::SnakeCaseValues,
    >,
    #[prost(oneof = "case_dependent_one_ofs::UnspecifiedCaseValues", tags = "5, 6")]
    pub unspecified_case_values: ::core::option::Option<
        case_dependent_one_ofs::UnspecifiedCaseValues,
    >,
}
/// Nested message and enum types in `CaseDependentOneOfs`.
pub mod case_dependent_one_ofs {
    #[actix_prost_macros::serde]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CamelCaseValues {
        #[prost(int64, tag = "1")]
        FirstCamelCaseValue(i64),
        #[prost(int64, tag = "2")]
        SecondCamelCaseValue(i64),
    }
    #[actix_prost_macros::serde]
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SnakeCaseValues {
        #[prost(int64, tag = "3")]
        FirstSnakeCaseValue(i64),
        #[prost(int64, tag = "4")]
        SecondSnakeCaseValue(i64),
    }
    #[actix_prost_macros::serde]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UnspecifiedCaseValues {
        #[prost(int64, tag = "5")]
        FirstUnspecifiedCaseValue(i64),
        #[prost(int64, tag = "6")]
        SecondUnspecifiedCaseValue(i64),
    }
}
pub mod serde_overrides_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::serde_overrides_rpc_server::SerdeOverridesRpc;
    use std::sync::Arc;
    #[actix_prost_macros::serde]
    #[actix_prost_macros::serde(rename_all = "camelCase")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CamelCaseSimpleMessagesRPCJson {
        #[prost(int64, tag = "1")]
        pub long_name_field: i64,
    }
    #[actix_prost_macros::serde]
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SnakeCaseSimpleMessagesRPCJson {
        #[prost(int64, tag = "1")]
        pub long_name_field: i64,
    }
    #[actix_prost_macros::serde]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnspecifiedCaseSimpleMessagesRPCJson {
        #[prost(int64, tag = "1")]
        pub long_name_field: i64,
    }
    #[actix_prost_macros::serde]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaseDependentOneOfsRPCJson {
        #[prost(oneof = "case_dependent_one_ofs::CamelCaseValues", tags = "1, 2")]
        pub camel_case_values: ::core::option::Option<
            case_dependent_one_ofs::CamelCaseValues,
        >,
        #[prost(oneof = "case_dependent_one_ofs::SnakeCaseValues", tags = "3, 4")]
        pub snake_case_values: ::core::option::Option<
            case_dependent_one_ofs::SnakeCaseValues,
        >,
        #[prost(oneof = "case_dependent_one_ofs::UnspecifiedCaseValues", tags = "5, 6")]
        pub unspecified_case_values: ::core::option::Option<
            case_dependent_one_ofs::UnspecifiedCaseValues,
        >,
    }
    async fn call_camel_case_simple_messages_rpc(
        service: ::actix_web::web::Data<dyn SerdeOverridesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<CamelCaseSimpleMessages>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            CamelCaseSimpleMessagesRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = CamelCaseSimpleMessages {
            long_name_field: json.long_name_field,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.camel_case_simple_messages_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_snake_case_simple_messages_rpc(
        service: ::actix_web::web::Data<dyn SerdeOverridesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<SnakeCaseSimpleMessages>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SnakeCaseSimpleMessagesRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SnakeCaseSimpleMessages {
            long_name_field: json.long_name_field,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.snake_case_simple_messages_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_unspecified_case_simple_messages_rpc(
        service: ::actix_web::web::Data<dyn SerdeOverridesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<UnspecifiedCaseSimpleMessages>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            UnspecifiedCaseSimpleMessagesRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = UnspecifiedCaseSimpleMessages {
            long_name_field: json.long_name_field,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.unspecified_case_simple_messages_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_case_dependent_one_ofs_rpc(
        service: ::actix_web::web::Data<dyn SerdeOverridesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<CaseDependentOneOfs>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            CaseDependentOneOfsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = CaseDependentOneOfs {
            camel_case_values: json.camel_case_values,
            snake_case_values: json.snake_case_values,
            unspecified_case_values: json.unspecified_case_values,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.case_dependent_one_ofs_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_serde_overrides_rpc(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn SerdeOverridesRpc + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config
            .route(
                "/serde-overrides/camel-case-simple-messages",
                ::actix_web::web::post().to(call_camel_case_simple_messages_rpc),
            );
        config
            .route(
                "/serde-overrides/snake-case-simple-messages",
                ::actix_web::web::post().to(call_snake_case_simple_messages_rpc),
            );
        config
            .route(
                "/serde-overrides/unspecified-case-simple-messages",
                ::actix_web::web::post().to(call_unspecified_case_simple_messages_rpc),
            );
        config
            .route(
                "/serde-overrides/case-dependent-oneofs",
                ::actix_web::web::post().to(call_case_dependent_one_ofs_rpc),
            );
    }
}
#[derive(Clone, Debug)]
pub struct CamelCaseSimpleMessagesInternal {
    pub long_name_field: i64,
}
impl convert_trait::TryConvert<CamelCaseSimpleMessages>
for CamelCaseSimpleMessagesInternal {
    fn try_convert(from: CamelCaseSimpleMessages) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
impl convert_trait::TryConvert<CamelCaseSimpleMessagesInternal>
for CamelCaseSimpleMessages {
    fn try_convert(from: CamelCaseSimpleMessagesInternal) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
#[derive(Clone, Debug)]
pub struct SnakeCaseSimpleMessagesInternal {
    pub long_name_field: i64,
}
impl convert_trait::TryConvert<SnakeCaseSimpleMessages>
for SnakeCaseSimpleMessagesInternal {
    fn try_convert(from: SnakeCaseSimpleMessages) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
impl convert_trait::TryConvert<SnakeCaseSimpleMessagesInternal>
for SnakeCaseSimpleMessages {
    fn try_convert(from: SnakeCaseSimpleMessagesInternal) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
#[derive(Clone, Debug)]
pub struct UnspecifiedCaseSimpleMessagesInternal {
    pub long_name_field: i64,
}
impl convert_trait::TryConvert<UnspecifiedCaseSimpleMessages>
for UnspecifiedCaseSimpleMessagesInternal {
    fn try_convert(from: UnspecifiedCaseSimpleMessages) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
impl convert_trait::TryConvert<UnspecifiedCaseSimpleMessagesInternal>
for UnspecifiedCaseSimpleMessages {
    fn try_convert(from: UnspecifiedCaseSimpleMessagesInternal) -> Result<Self, String> {
        Ok(Self {
            long_name_field: from.long_name_field,
        })
    }
}
#[derive(Clone, Debug)]
pub struct CaseDependentOneOfsInternal {
    pub camel_case_values: ::core::option::Option<
        case_dependent_one_ofs::CamelCaseValues,
    >,
    pub snake_case_values: ::core::option::Option<
        case_dependent_one_ofs::SnakeCaseValues,
    >,
    pub unspecified_case_values: ::core::option::Option<
        case_dependent_one_ofs::UnspecifiedCaseValues,
    >,
}
impl convert_trait::TryConvert<CaseDependentOneOfs> for CaseDependentOneOfsInternal {
    fn try_convert(from: CaseDependentOneOfs) -> Result<Self, String> {
        Ok(Self {
            camel_case_values: from.camel_case_values,
            snake_case_values: from.snake_case_values,
            unspecified_case_values: from.unspecified_case_values,
        })
    }
}
impl convert_trait::TryConvert<CaseDependentOneOfsInternal> for CaseDependentOneOfs {
    fn try_convert(from: CaseDependentOneOfsInternal) -> Result<Self, String> {
        Ok(Self {
            camel_case_values: from.camel_case_values,
            snake_case_values: from.snake_case_values,
            unspecified_case_values: from.unspecified_case_values,
        })
    }
}
/// Generated client implementations.
pub mod serde_overrides_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SerdeOverridesRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SerdeOverridesRpcClient<tonic::transport::Channel> {
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
    impl<T> SerdeOverridesRpcClient<T>
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
        ) -> SerdeOverridesRpcClient<InterceptedService<T, F>>
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
            SerdeOverridesRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn camel_case_simple_messages_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::CamelCaseSimpleMessages>,
        ) -> Result<tonic::Response<super::CamelCaseSimpleMessages>, tonic::Status> {
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
                "/serde_overrides.SerdeOverridesRPC/CamelCaseSimpleMessagesRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn snake_case_simple_messages_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::SnakeCaseSimpleMessages>,
        ) -> Result<tonic::Response<super::SnakeCaseSimpleMessages>, tonic::Status> {
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
                "/serde_overrides.SerdeOverridesRPC/SnakeCaseSimpleMessagesRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn unspecified_case_simple_messages_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::UnspecifiedCaseSimpleMessages>,
        ) -> Result<
            tonic::Response<super::UnspecifiedCaseSimpleMessages>,
            tonic::Status,
        > {
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
                "/serde_overrides.SerdeOverridesRPC/UnspecifiedCaseSimpleMessagesRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn case_dependent_one_ofs_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::CaseDependentOneOfs>,
        ) -> Result<tonic::Response<super::CaseDependentOneOfs>, tonic::Status> {
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
                "/serde_overrides.SerdeOverridesRPC/CaseDependentOneOfsRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod serde_overrides_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SerdeOverridesRpcServer.
    #[async_trait]
    pub trait SerdeOverridesRpc: Send + Sync + 'static {
        async fn camel_case_simple_messages_rpc(
            &self,
            request: tonic::Request<super::CamelCaseSimpleMessages>,
        ) -> Result<tonic::Response<super::CamelCaseSimpleMessages>, tonic::Status>;
        async fn snake_case_simple_messages_rpc(
            &self,
            request: tonic::Request<super::SnakeCaseSimpleMessages>,
        ) -> Result<tonic::Response<super::SnakeCaseSimpleMessages>, tonic::Status>;
        async fn unspecified_case_simple_messages_rpc(
            &self,
            request: tonic::Request<super::UnspecifiedCaseSimpleMessages>,
        ) -> Result<
            tonic::Response<super::UnspecifiedCaseSimpleMessages>,
            tonic::Status,
        >;
        async fn case_dependent_one_ofs_rpc(
            &self,
            request: tonic::Request<super::CaseDependentOneOfs>,
        ) -> Result<tonic::Response<super::CaseDependentOneOfs>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SerdeOverridesRpcServer<T: SerdeOverridesRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SerdeOverridesRpc> SerdeOverridesRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SerdeOverridesRpcServer<T>
    where
        T: SerdeOverridesRpc,
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
                "/serde_overrides.SerdeOverridesRPC/CamelCaseSimpleMessagesRPC" => {
                    #[allow(non_camel_case_types)]
                    struct CamelCaseSimpleMessagesRPCSvc<T: SerdeOverridesRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SerdeOverridesRpc,
                    > tonic::server::UnaryService<super::CamelCaseSimpleMessages>
                    for CamelCaseSimpleMessagesRPCSvc<T> {
                        type Response = super::CamelCaseSimpleMessages;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CamelCaseSimpleMessages>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).camel_case_simple_messages_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CamelCaseSimpleMessagesRPCSvc(inner);
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
                "/serde_overrides.SerdeOverridesRPC/SnakeCaseSimpleMessagesRPC" => {
                    #[allow(non_camel_case_types)]
                    struct SnakeCaseSimpleMessagesRPCSvc<T: SerdeOverridesRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SerdeOverridesRpc,
                    > tonic::server::UnaryService<super::SnakeCaseSimpleMessages>
                    for SnakeCaseSimpleMessagesRPCSvc<T> {
                        type Response = super::SnakeCaseSimpleMessages;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SnakeCaseSimpleMessages>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).snake_case_simple_messages_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SnakeCaseSimpleMessagesRPCSvc(inner);
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
                "/serde_overrides.SerdeOverridesRPC/UnspecifiedCaseSimpleMessagesRPC" => {
                    #[allow(non_camel_case_types)]
                    struct UnspecifiedCaseSimpleMessagesRPCSvc<T: SerdeOverridesRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SerdeOverridesRpc,
                    > tonic::server::UnaryService<super::UnspecifiedCaseSimpleMessages>
                    for UnspecifiedCaseSimpleMessagesRPCSvc<T> {
                        type Response = super::UnspecifiedCaseSimpleMessages;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnspecifiedCaseSimpleMessages>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).unspecified_case_simple_messages_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnspecifiedCaseSimpleMessagesRPCSvc(inner);
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
                "/serde_overrides.SerdeOverridesRPC/CaseDependentOneOfsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct CaseDependentOneOfsRPCSvc<T: SerdeOverridesRpc>(pub Arc<T>);
                    impl<
                        T: SerdeOverridesRpc,
                    > tonic::server::UnaryService<super::CaseDependentOneOfs>
                    for CaseDependentOneOfsRPCSvc<T> {
                        type Response = super::CaseDependentOneOfs;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CaseDependentOneOfs>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).case_dependent_one_ofs_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CaseDependentOneOfsRPCSvc(inner);
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
    impl<T: SerdeOverridesRpc> Clone for SerdeOverridesRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SerdeOverridesRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SerdeOverridesRpc> tonic::server::NamedService
    for SerdeOverridesRpcServer<T> {
        const NAME: &'static str = "serde_overrides.SerdeOverridesRPC";
    }
}

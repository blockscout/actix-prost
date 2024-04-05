#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nested {
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapValue {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionsRequest {
    #[prost(map = "string, message", tag = "1")]
    pub map_field: ::std::collections::HashMap<::prost::alloc::string::String, MapValue>,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "conversions_request::NestedEnum", tag = "4")]
    pub nested_enum: i32,
    #[prost(message, optional, tag = "5")]
    pub nested: ::core::option::Option<Nested>,
}
/// Nested message and enum types in `ConversionsRequest`.
pub mod conversions_request {
    #[actix_prost_macros::serde]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum NestedEnum {
        NestedOk = 0,
        NestedError = 1,
    }
    impl NestedEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NestedEnum::NestedOk => "NESTED_OK",
                NestedEnum::NestedError => "NESTED_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NESTED_OK" => Some(Self::NestedOk),
                "NESTED_ERROR" => Some(Self::NestedError),
                _ => None,
            }
        }
    }
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionsResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub nested: ::core::option::Option<Nested>,
    #[prost(map = "string, message", tag = "3")]
    pub map_field: ::std::collections::HashMap<::prost::alloc::string::String, MapValue>,
}
pub mod conversions_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::conversions_rpc_server::ConversionsRpc;
    use std::sync::Arc;
    #[actix_prost_macros::serde]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConvertRPCJson {
        #[prost(map = "string, message", tag = "1")]
        pub map_field: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            MapValue,
        >,
        #[prost(string, tag = "2")]
        pub query: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "3")]
        pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(enumeration = "conversions_request::NestedEnum", tag = "4")]
        pub nested_enum: i32,
        #[prost(message, optional, tag = "5")]
        pub nested: ::core::option::Option<Nested>,
    }
    async fn call_convert_rpc(
        service: ::actix_web::web::Data<dyn ConversionsRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<ConversionsResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ConvertRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ConversionsRequest {
            map_field: json.map_field,
            query: json.query,
            addresses: json.addresses,
            nested_enum: json.nested_enum,
            nested: json.nested,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.convert_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_conversions_rpc(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn ConversionsRpc + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route("/conversions", ::actix_web::web::post().to(call_convert_rpc));
    }
}
#[derive(serde::Serialize)]
#[derive(Clone, Debug)]
pub struct MapValueInternal {
    pub address: ethers::types::Address,
}
impl convert_trait::TryConvert<MapValue> for MapValueInternal {
    fn try_convert(from: MapValue) -> Result<Self, String> {
        Ok(Self {
            address: convert_trait::TryConvert::try_convert(from.address)?,
        })
    }
}
#[derive(serde::Serialize)]
#[derive(Clone, Debug)]
pub struct NestedInternal {
    pub address: ethers::types::Address,
}
impl convert_trait::TryConvert<Nested> for NestedInternal {
    fn try_convert(from: Nested) -> Result<Self, String> {
        Ok(Self {
            address: convert_trait::TryConvert::try_convert(from.address)?,
        })
    }
}
#[derive(serde::Serialize)]
#[derive(Clone, Debug)]
pub struct ConversionsRequestInternal {
    pub map_field: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        MapValueInternal,
    >,
    pub query: ::prost::alloc::string::String,
    pub addresses: std::collections::HashSet<ethers::types::Address>,
    pub nested_enum: conversions_request::NestedEnum,
    pub nested: NestedInternal,
    pub field1: Option<String>,
    pub field2: Option<i32>,
}
impl convert_trait::TryConvert<ConversionsRequest> for ConversionsRequestInternal {
    fn try_convert(from: ConversionsRequest) -> Result<Self, String> {
        Ok(Self {
            map_field: convert_trait::TryConvert::try_convert(from.map_field)?,
            query: Default::default(),
            addresses: convert_trait::TryConvert::try_convert(from.addresses)?,
            nested_enum: conversions_request::NestedEnum::try_from(from.nested_enum)?,
            nested: convert_trait::TryConvert::try_convert(
                from.nested.ok_or("field nested is required")?,
            )?,
            field1: None,
            field2: None,
        })
    }
}
impl convert_trait::TryConvert<NestedInternal> for Nested {
    fn try_convert(from: NestedInternal) -> Result<Self, String> {
        Ok(Self {
            address: convert_trait::TryConvert::try_convert(from.address)?,
        })
    }
}
impl convert_trait::TryConvert<MapValueInternal> for MapValue {
    fn try_convert(from: MapValueInternal) -> Result<Self, String> {
        Ok(Self {
            address: convert_trait::TryConvert::try_convert(from.address)?,
        })
    }
}
#[derive(Clone, Debug)]
pub struct ConversionsResponseInternal {
    pub address: ethers::types::Address,
    pub nested: ::core::option::Option<NestedInternal>,
    pub map_field: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        MapValueInternal,
    >,
}
impl convert_trait::TryConvert<ConversionsResponseInternal> for ConversionsResponse {
    fn try_convert(from: ConversionsResponseInternal) -> Result<Self, String> {
        Ok(Self {
            address: convert_trait::TryConvert::try_convert(from.address)?,
            nested: convert_trait::TryConvert::try_convert(from.nested)?,
            map_field: convert_trait::TryConvert::try_convert(from.map_field)?,
        })
    }
}
/// Generated client implementations.
pub mod conversions_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ConversionsRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConversionsRpcClient<tonic::transport::Channel> {
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
    impl<T> ConversionsRpcClient<T>
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
        ) -> ConversionsRpcClient<InterceptedService<T, F>>
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
            ConversionsRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn convert_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::ConversionsRequest>,
        ) -> Result<tonic::Response<super::ConversionsResponse>, tonic::Status> {
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
                "/conversions.ConversionsRPC/ConvertRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod conversions_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ConversionsRpcServer.
    #[async_trait]
    pub trait ConversionsRpc: Send + Sync + 'static {
        async fn convert_rpc(
            &self,
            request: tonic::Request<super::ConversionsRequest>,
        ) -> Result<tonic::Response<super::ConversionsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ConversionsRpcServer<T: ConversionsRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ConversionsRpc> ConversionsRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ConversionsRpcServer<T>
    where
        T: ConversionsRpc,
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
                "/conversions.ConversionsRPC/ConvertRPC" => {
                    #[allow(non_camel_case_types)]
                    struct ConvertRPCSvc<T: ConversionsRpc>(pub Arc<T>);
                    impl<
                        T: ConversionsRpc,
                    > tonic::server::UnaryService<super::ConversionsRequest>
                    for ConvertRPCSvc<T> {
                        type Response = super::ConversionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConversionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).convert_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConvertRPCSvc(inner);
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
    impl<T: ConversionsRpc> Clone for ConversionsRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ConversionsRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ConversionsRpc> tonic::server::NamedService for ConversionsRpcServer<T> {
        const NAME: &'static str = "conversions.ConversionsRPC";
    }
}

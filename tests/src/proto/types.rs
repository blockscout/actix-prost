#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scalars {
    #[prost(double, tag = "1")]
    pub a: f64,
    #[prost(int64, tag = "2")]
    pub b: i64,
    #[prost(string, tag = "3")]
    pub c: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", tag = "4")]
    pub d: ::prost::bytes::Bytes,
    #[prost(bool, tag = "5")]
    pub e: bool,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalScalars {
    #[prost(double, optional, tag = "1")]
    pub a: ::core::option::Option<f64>,
    #[prost(int64, optional, tag = "2")]
    pub b: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub c: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "bytes", optional, tag = "4")]
    pub d: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(bool, optional, tag = "5")]
    pub e: ::core::option::Option<bool>,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Enums {
    #[prost(enumeration = "Values", tag = "1")]
    pub values: i32,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEnums {
    #[prost(enumeration = "Values", optional, tag = "1")]
    pub values: ::core::option::Option<i32>,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repeated {
    #[prost(string, repeated, tag = "1")]
    pub foo: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Maps {
    #[prost(map = "string, int32", tag = "1")]
    pub foo: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneOfs {
    #[prost(oneof = "one_ofs::Values", tags = "1, 2, 3")]
    pub values: ::core::option::Option<one_ofs::Values>,
}
/// Nested message and enum types in `OneOfs`.
pub mod one_ofs {
    #[actix_prost_macros::serde]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Values {
        #[prost(string, tag = "1")]
        Foo(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        Bar(::prost::bytes::Bytes),
        #[prost(int64, tag = "3")]
        Baz(i64),
    }
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Complex {
    #[prost(message, optional, tag = "1")]
    pub scalars: ::core::option::Option<Scalars>,
    #[prost(message, optional, tag = "2")]
    pub enums: ::core::option::Option<Enums>,
    #[prost(message, optional, tag = "3")]
    pub repeated: ::core::option::Option<Repeated>,
    #[prost(message, optional, tag = "4")]
    pub maps: ::core::option::Option<Maps>,
    /// Google google = 6;
    #[prost(message, optional, tag = "5")]
    pub oneofs: ::core::option::Option<OneOfs>,
}
#[actix_prost_macros::serde]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Values {
    Foo = 0,
    Bar = 1,
}
impl Values {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Values::Foo => "FOO",
            Values::Bar => "BAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FOO" => Some(Self::Foo),
            "BAR" => Some(Self::Bar),
            _ => None,
        }
    }
}
pub mod types_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::types_rpc_server::TypesRpc;
    use std::sync::Arc;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct ScalarsRPCJson {
        #[prost(double, tag = "1")]
        pub a: f64,
        #[prost(int64, tag = "2")]
        pub b: i64,
        #[prost(string, tag = "3")]
        pub c: ::prost::alloc::string::String,
        #[prost(bytes = "bytes", tag = "4")]
        pub d: ::prost::bytes::Bytes,
        #[prost(bool, tag = "5")]
        pub e: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct OptionalScalarsRPCJson {
        #[prost(double, optional, tag = "1")]
        pub a: ::core::option::Option<f64>,
        #[prost(int64, optional, tag = "2")]
        pub b: ::core::option::Option<i64>,
        #[prost(string, optional, tag = "3")]
        pub c: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "bytes", optional, tag = "4")]
        pub d: ::core::option::Option<::prost::bytes::Bytes>,
        #[prost(bool, optional, tag = "5")]
        pub e: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct EnumsRPCJson {
        #[prost(enumeration = "Values", tag = "1")]
        pub values: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct OptionalEnumsRPCJson {
        #[prost(enumeration = "Values", optional, tag = "1")]
        pub values: ::core::option::Option<i32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct RepeatedRPCJson {
        #[prost(string, repeated, tag = "1")]
        pub foo: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct MapsRPCJson {
        #[prost(map = "string, int32", tag = "1")]
        pub foo: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct OneOfsRPCJson {
        #[prost(oneof = "one_ofs::Values", tags = "1, 2, 3")]
        pub values: ::core::option::Option<one_ofs::Values>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct ComplexRPCJson {
        #[prost(message, optional, tag = "1")]
        pub scalars: ::core::option::Option<Scalars>,
        #[prost(message, optional, tag = "2")]
        pub enums: ::core::option::Option<Enums>,
        #[prost(message, optional, tag = "3")]
        pub repeated: ::core::option::Option<Repeated>,
        #[prost(message, optional, tag = "4")]
        pub maps: ::core::option::Option<Maps>,
        /// Google google = 6;
        #[prost(message, optional, tag = "5")]
        pub oneofs: ::core::option::Option<OneOfs>,
    }
    async fn call_scalars_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Scalars>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            ScalarsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Scalars {
            a: json.a,
            b: json.b,
            c: json.c,
            d: json.d,
            e: json.e,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.scalars_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_optional_scalars_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<OptionalScalars>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            OptionalScalarsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = OptionalScalars {
            a: json.a,
            b: json.b,
            c: json.c,
            d: json.d,
            e: json.e,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.optional_scalars_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_enums_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Enums>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            EnumsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Enums { values: json.values };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.enums_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_optional_enums_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<OptionalEnums>, ::actix_web::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            OptionalEnumsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await?
            .into_inner();
        let request = OptionalEnums {
            values: json.values,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service
            .optional_enums_rpc(request)
            .await
            .map_err(::actix_prost::map_tonic_error)?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_repeated_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Repeated>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            RepeatedRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Repeated { foo: json.foo };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.repeated_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_maps_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Maps>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            MapsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Maps { foo: json.foo };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.maps_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_one_ofs_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<OneOfs>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            OneOfsRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = OneOfs { values: json.values };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.one_ofs_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_complex_rpc(
        service: ::actix_web::web::Data<dyn TypesRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<Complex>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json::<
            ComplexRPCJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Complex {
            scalars: json.scalars,
            enums: json.enums,
            repeated: json.repeated,
            maps: json.maps,
            oneofs: json.oneofs,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.complex_rpc(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_types_rpc(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn TypesRpc + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route("/types/scalars", ::actix_web::web::post().to(call_scalars_rpc));
        config
            .route(
                "/types/optional_scalars",
                ::actix_web::web::post().to(call_optional_scalars_rpc),
            );
        config.route("/types/enums", ::actix_web::web::post().to(call_enums_rpc));
        config
            .route(
                "/types/optional_enums",
                ::actix_web::web::post().to(call_optional_enums_rpc),
            );
        config.route("/types/repeated", ::actix_web::web::post().to(call_repeated_rpc));
        config.route("/types/maps", ::actix_web::web::post().to(call_maps_rpc));
        config.route("/types/oneofs", ::actix_web::web::post().to(call_one_ofs_rpc));
        config.route("/types/complex", ::actix_web::web::post().to(call_complex_rpc));
    }
}
/// Generated client implementations.
pub mod types_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TypesRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TypesRpcClient<tonic::transport::Channel> {
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
    impl<T> TypesRpcClient<T>
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
        ) -> TypesRpcClient<InterceptedService<T, F>>
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
            TypesRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn scalars_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Scalars>,
        ) -> Result<tonic::Response<super::Scalars>, tonic::Status> {
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
                "/types.TypesRPC/ScalarsRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn optional_scalars_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::OptionalScalars>,
        ) -> Result<tonic::Response<super::OptionalScalars>, tonic::Status> {
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
                "/types.TypesRPC/OptionalScalarsRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn enums_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Enums>,
        ) -> Result<tonic::Response<super::Enums>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/types.TypesRPC/EnumsRPC");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn optional_enums_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::OptionalEnums>,
        ) -> Result<tonic::Response<super::OptionalEnums>, tonic::Status> {
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
                "/types.TypesRPC/OptionalEnumsRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn repeated_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Repeated>,
        ) -> Result<tonic::Response<super::Repeated>, tonic::Status> {
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
                "/types.TypesRPC/RepeatedRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn maps_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Maps>,
        ) -> Result<tonic::Response<super::Maps>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/types.TypesRPC/MapsRPC");
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
            let path = http::uri::PathAndQuery::from_static("/types.TypesRPC/OneOfsRPC");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// rpc GoogleRPC(Google) returns (Google);
        pub async fn complex_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::Complex>,
        ) -> Result<tonic::Response<super::Complex>, tonic::Status> {
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
                "/types.TypesRPC/ComplexRPC",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod types_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TypesRpcServer.
    #[async_trait]
    pub trait TypesRpc: Send + Sync + 'static {
        async fn scalars_rpc(
            &self,
            request: tonic::Request<super::Scalars>,
        ) -> Result<tonic::Response<super::Scalars>, tonic::Status>;
        async fn optional_scalars_rpc(
            &self,
            request: tonic::Request<super::OptionalScalars>,
        ) -> Result<tonic::Response<super::OptionalScalars>, tonic::Status>;
        async fn enums_rpc(
            &self,
            request: tonic::Request<super::Enums>,
        ) -> Result<tonic::Response<super::Enums>, tonic::Status>;
        async fn optional_enums_rpc(
            &self,
            request: tonic::Request<super::OptionalEnums>,
        ) -> Result<tonic::Response<super::OptionalEnums>, tonic::Status>;
        async fn repeated_rpc(
            &self,
            request: tonic::Request<super::Repeated>,
        ) -> Result<tonic::Response<super::Repeated>, tonic::Status>;
        async fn maps_rpc(
            &self,
            request: tonic::Request<super::Maps>,
        ) -> Result<tonic::Response<super::Maps>, tonic::Status>;
        async fn one_ofs_rpc(
            &self,
            request: tonic::Request<super::OneOfs>,
        ) -> Result<tonic::Response<super::OneOfs>, tonic::Status>;
        /// rpc GoogleRPC(Google) returns (Google);
        async fn complex_rpc(
            &self,
            request: tonic::Request<super::Complex>,
        ) -> Result<tonic::Response<super::Complex>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TypesRpcServer<T: TypesRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TypesRpc> TypesRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TypesRpcServer<T>
    where
        T: TypesRpc,
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
                "/types.TypesRPC/ScalarsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct ScalarsRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::Scalars>
                    for ScalarsRPCSvc<T> {
                        type Response = super::Scalars;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Scalars>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).scalars_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScalarsRPCSvc(inner);
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
                "/types.TypesRPC/OptionalScalarsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct OptionalScalarsRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::OptionalScalars>
                    for OptionalScalarsRPCSvc<T> {
                        type Response = super::OptionalScalars;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OptionalScalars>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).optional_scalars_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OptionalScalarsRPCSvc(inner);
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
                "/types.TypesRPC/EnumsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct EnumsRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::Enums>
                    for EnumsRPCSvc<T> {
                        type Response = super::Enums;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Enums>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).enums_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EnumsRPCSvc(inner);
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
                "/types.TypesRPC/OptionalEnumsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct OptionalEnumsRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::OptionalEnums>
                    for OptionalEnumsRPCSvc<T> {
                        type Response = super::OptionalEnums;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OptionalEnums>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).optional_enums_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OptionalEnumsRPCSvc(inner);
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
                "/types.TypesRPC/RepeatedRPC" => {
                    #[allow(non_camel_case_types)]
                    struct RepeatedRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::Repeated>
                    for RepeatedRPCSvc<T> {
                        type Response = super::Repeated;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Repeated>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).repeated_rpc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RepeatedRPCSvc(inner);
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
                "/types.TypesRPC/MapsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct MapsRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::Maps>
                    for MapsRPCSvc<T> {
                        type Response = super::Maps;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Maps>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).maps_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MapsRPCSvc(inner);
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
                "/types.TypesRPC/OneOfsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct OneOfsRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::OneOfs>
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
                "/types.TypesRPC/ComplexRPC" => {
                    #[allow(non_camel_case_types)]
                    struct ComplexRPCSvc<T: TypesRpc>(pub Arc<T>);
                    impl<T: TypesRpc> tonic::server::UnaryService<super::Complex>
                    for ComplexRPCSvc<T> {
                        type Response = super::Complex;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Complex>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).complex_rpc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ComplexRPCSvc(inner);
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
    impl<T: TypesRpc> Clone for TypesRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TypesRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TypesRpc> tonic::server::NamedService for TypesRpcServer<T> {
        const NAME: &'static str = "types.TypesRPC";
    }
}

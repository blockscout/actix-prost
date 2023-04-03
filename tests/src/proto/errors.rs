#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorRequest {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {}
pub mod errors_rpc_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::{errors_rpc_server::ErrorsRpc, *};
    use std::sync::Arc;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    pub struct ErrorPath {
        #[prost(int32, tag = "1")]
        pub code: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct ErrorQuery {
        #[prost(string, tag = "2")]
        pub query: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[actix_prost_macros::serde]
    pub struct ErrorJson {
        #[prost(string, tag = "3")]
        pub message: ::prost::alloc::string::String,
    }
    async fn call_error(
        service: ::actix_web::web::Data<dyn ErrorsRpc + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<ErrorResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let path =
            <::actix_web::web::Path<ErrorPath> as ::actix_web::FromRequest>::extract(&http_request)
                .await
                .map_err(|err| {
                    ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument)
                })?
                .into_inner();
        let query = <::actix_web::web::Query<ErrorQuery> as ::actix_web::FromRequest>::extract(
            &http_request,
        )
        .await
        .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
        .into_inner();
        let json = <::actix_web::web::Json<ErrorJson> as ::actix_web::FromRequest>::from_request(
            &http_request,
            &mut payload,
        )
        .await
        .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
        .into_inner();
        let request = ErrorRequest {
            code: path.code,
            query: query.query,
            message: json.message,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.error(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_errors_rpc(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn ErrorsRpc + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route("/errors/{code}", ::actix_web::web::post().to(call_error));
    }
}
/// Generated client implementations.
pub mod errors_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::{http::Uri, *};
    #[derive(Debug, Clone)]
    pub struct ErrorsRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ErrorsRpcClient<tonic::transport::Channel> {
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
    impl<T> ErrorsRpcClient<T>
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
        ) -> ErrorsRpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ErrorsRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn error(
            &mut self,
            request: impl tonic::IntoRequest<super::ErrorRequest>,
        ) -> Result<tonic::Response<super::ErrorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/errors.ErrorsRPC/Error");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod errors_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ErrorsRpcServer.
    #[async_trait]
    pub trait ErrorsRpc: Send + Sync + 'static {
        async fn error(
            &self,
            request: tonic::Request<super::ErrorRequest>,
        ) -> Result<tonic::Response<super::ErrorResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ErrorsRpcServer<T: ErrorsRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ErrorsRpc> ErrorsRpcServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ErrorsRpcServer<T>
    where
        T: ErrorsRpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/errors.ErrorsRPC/Error" => {
                    #[allow(non_camel_case_types)]
                    struct ErrorSvc<T: ErrorsRpc>(pub Arc<T>);
                    impl<T: ErrorsRpc> tonic::server::UnaryService<super::ErrorRequest> for ErrorSvc<T> {
                        type Response = super::ErrorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ErrorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).error(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ErrorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ErrorsRpc> Clone for ErrorsRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ErrorsRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ErrorsRpc> tonic::server::NamedService for ErrorsRpcServer<T> {
        const NAME: &'static str = "errors.ErrorsRPC";
    }
}

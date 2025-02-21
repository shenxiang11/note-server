// This file is @generated by prost-build.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SaveCountRequest {
    #[prost(enumeration = "CountBiz", tag = "1")]
    pub biz: i32,
    #[prost(int64, tag = "2")]
    pub biz_id: i64,
    #[prost(int64, tag = "3")]
    pub n: i64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SaveCountResponse {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetCountRequest {
    #[prost(enumeration = "CountBiz", tag = "1")]
    pub biz: i32,
    #[prost(int64, tag = "2")]
    pub biz_id: i64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetCountResponse {
    #[prost(int64, tag = "1")]
    pub count: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetCountRequest {
    #[prost(enumeration = "CountBiz", tag = "1")]
    pub biz: i32,
    #[prost(int64, repeated, tag = "2")]
    pub biz_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetCountResponse {
    #[prost(map = "int64, int64", tag = "1")]
    pub counts: ::std::collections::HashMap<i64, i64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LikeRequest {
    #[prost(enumeration = "UserLikesBiz", tag = "1")]
    pub biz: i32,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(int64, tag = "3")]
    pub biz_id: i64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LikeResponse {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct UnlikeRequest {
    #[prost(enumeration = "UserLikesBiz", tag = "1")]
    pub biz: i32,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(int64, tag = "3")]
    pub biz_id: i64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct UnlikeResponse {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BizIdsAndUserIds {
    #[prost(int64, tag = "1")]
    pub biz_id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetIsLikedRequest {
    #[prost(enumeration = "UserLikesBiz", tag = "1")]
    pub biz: i32,
    #[prost(message, repeated, tag = "2")]
    pub query: ::prost::alloc::vec::Vec<BizIdsAndUserIds>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BizIdsAndUserIdsAndIsLiked {
    #[prost(int64, tag = "1")]
    pub biz_id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(bool, tag = "3")]
    pub is_liked: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetIsLikedResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<BizIdsAndUserIdsAndIsLiked>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CountBiz {
    CountUnknown = 0,
    CountNoteRead = 1,
    CountNoteLike = 2,
    CountNoteCollect = 3,
    CountNoteComment = 4,
}
impl CountBiz {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::CountUnknown => "COUNT_UNKNOWN",
            Self::CountNoteRead => "COUNT_NOTE_READ",
            Self::CountNoteLike => "COUNT_NOTE_LIKE",
            Self::CountNoteCollect => "COUNT_NOTE_COLLECT",
            Self::CountNoteComment => "COUNT_NOTE_COMMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COUNT_UNKNOWN" => Some(Self::CountUnknown),
            "COUNT_NOTE_READ" => Some(Self::CountNoteRead),
            "COUNT_NOTE_LIKE" => Some(Self::CountNoteLike),
            "COUNT_NOTE_COLLECT" => Some(Self::CountNoteCollect),
            "COUNT_NOTE_COMMENT" => Some(Self::CountNoteComment),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserLikesBiz {
    UserLikesUnknown = 0,
    UserLikesNote = 1,
    UserLikesComment = 2,
}
impl UserLikesBiz {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::UserLikesUnknown => "USER_LIKES_UNKNOWN",
            Self::UserLikesNote => "USER_LIKES_NOTE",
            Self::UserLikesComment => "USER_LIKES_COMMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_LIKES_UNKNOWN" => Some(Self::UserLikesUnknown),
            "USER_LIKES_NOTE" => Some(Self::UserLikesNote),
            "USER_LIKES_COMMENT" => Some(Self::UserLikesComment),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod interactive_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InteractiveServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InteractiveServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InteractiveServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> InteractiveServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            InteractiveServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn save_count(
            &mut self,
            request: impl tonic::IntoRequest<super::SaveCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SaveCountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/interactive.InteractiveService/SaveCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("interactive.InteractiveService", "SaveCount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_count(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/interactive.InteractiveService/GetCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("interactive.InteractiveService", "GetCount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_get_count(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetCountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/interactive.InteractiveService/BatchGetCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("interactive.InteractiveService", "BatchGetCount"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn like(
            &mut self,
            request: impl tonic::IntoRequest<super::LikeRequest>,
        ) -> std::result::Result<tonic::Response<super::LikeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/interactive.InteractiveService/Like",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("interactive.InteractiveService", "Like"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlike(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlikeRequest>,
        ) -> std::result::Result<tonic::Response<super::UnlikeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/interactive.InteractiveService/Unlike",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("interactive.InteractiveService", "Unlike"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_get_is_liked(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetIsLikedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetIsLikedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/interactive.InteractiveService/BatchGetIsLiked",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("interactive.InteractiveService", "BatchGetIsLiked"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod interactive_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InteractiveServiceServer.
    #[async_trait]
    pub trait InteractiveService: std::marker::Send + std::marker::Sync + 'static {
        async fn save_count(
            &self,
            request: tonic::Request<super::SaveCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SaveCountResponse>,
            tonic::Status,
        >;
        async fn get_count(
            &self,
            request: tonic::Request<super::GetCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCountResponse>,
            tonic::Status,
        >;
        async fn batch_get_count(
            &self,
            request: tonic::Request<super::BatchGetCountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetCountResponse>,
            tonic::Status,
        >;
        async fn like(
            &self,
            request: tonic::Request<super::LikeRequest>,
        ) -> std::result::Result<tonic::Response<super::LikeResponse>, tonic::Status>;
        async fn unlike(
            &self,
            request: tonic::Request<super::UnlikeRequest>,
        ) -> std::result::Result<tonic::Response<super::UnlikeResponse>, tonic::Status>;
        async fn batch_get_is_liked(
            &self,
            request: tonic::Request<super::BatchGetIsLikedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchGetIsLikedResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct InteractiveServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> InteractiveServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InteractiveServiceServer<T>
    where
        T: InteractiveService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/interactive.InteractiveService/SaveCount" => {
                    #[allow(non_camel_case_types)]
                    struct SaveCountSvc<T: InteractiveService>(pub Arc<T>);
                    impl<
                        T: InteractiveService,
                    > tonic::server::UnaryService<super::SaveCountRequest>
                    for SaveCountSvc<T> {
                        type Response = super::SaveCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SaveCountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InteractiveService>::save_count(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SaveCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/interactive.InteractiveService/GetCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetCountSvc<T: InteractiveService>(pub Arc<T>);
                    impl<
                        T: InteractiveService,
                    > tonic::server::UnaryService<super::GetCountRequest>
                    for GetCountSvc<T> {
                        type Response = super::GetCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InteractiveService>::get_count(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/interactive.InteractiveService/BatchGetCount" => {
                    #[allow(non_camel_case_types)]
                    struct BatchGetCountSvc<T: InteractiveService>(pub Arc<T>);
                    impl<
                        T: InteractiveService,
                    > tonic::server::UnaryService<super::BatchGetCountRequest>
                    for BatchGetCountSvc<T> {
                        type Response = super::BatchGetCountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchGetCountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InteractiveService>::batch_get_count(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BatchGetCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/interactive.InteractiveService/Like" => {
                    #[allow(non_camel_case_types)]
                    struct LikeSvc<T: InteractiveService>(pub Arc<T>);
                    impl<
                        T: InteractiveService,
                    > tonic::server::UnaryService<super::LikeRequest> for LikeSvc<T> {
                        type Response = super::LikeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LikeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InteractiveService>::like(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LikeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/interactive.InteractiveService/Unlike" => {
                    #[allow(non_camel_case_types)]
                    struct UnlikeSvc<T: InteractiveService>(pub Arc<T>);
                    impl<
                        T: InteractiveService,
                    > tonic::server::UnaryService<super::UnlikeRequest>
                    for UnlikeSvc<T> {
                        type Response = super::UnlikeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnlikeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InteractiveService>::unlike(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnlikeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/interactive.InteractiveService/BatchGetIsLiked" => {
                    #[allow(non_camel_case_types)]
                    struct BatchGetIsLikedSvc<T: InteractiveService>(pub Arc<T>);
                    impl<
                        T: InteractiveService,
                    > tonic::server::UnaryService<super::BatchGetIsLikedRequest>
                    for BatchGetIsLikedSvc<T> {
                        type Response = super::BatchGetIsLikedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchGetIsLikedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InteractiveService>::batch_get_is_liked(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BatchGetIsLikedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for InteractiveServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "interactive.InteractiveService";
    impl<T> tonic::server::NamedService for InteractiveServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}

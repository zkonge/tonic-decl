#[doc(hidden)]
#[macro_export]
macro_rules! __define_client {
    (
        [$(#[$gate:meta])*]
        $(#[$client_attr:meta])*
        $client_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ );
        )*
    ) => {
        $(#[$gate])*
        $(#[$client_attr])*
        #[derive(::core::fmt::Debug, ::core::clone::Clone)]
        pub struct $client_name<T> {
            inner: $crate::__private::tonic::client::Grpc<T>,
        }

        $(#[$gate])*
        impl<T> $client_name<T>
        where
            T: $crate::__private::tonic::client::GrpcService<$crate::__private::tonic::body::Body>,
            T::Error: ::core::convert::Into<$crate::__private::StdError>,
            T::ResponseBody: $crate::__private::http_body::Body<Data = $crate::__private::prost::bytes::Bytes>
                + ::core::marker::Send
                + 'static,
            <T::ResponseBody as $crate::__private::http_body::Body>::Error:
                ::core::convert::Into<$crate::__private::StdError> + ::core::marker::Send,
        {
            /// Creates a client that sends requests through `inner`, e.g. a `tonic::transport::Channel`.
            pub fn new(inner: T) -> Self {
                let inner = $crate::__private::tonic::client::Grpc::new(inner);
                Self { inner }
            }

            /// Creates a client that sends requests through `inner`, using the scheme and authority
            /// of `origin`.
            pub fn with_origin(inner: T, origin: $crate::__private::http::uri::Uri) -> Self {
                let inner = $crate::__private::tonic::client::Grpc::with_origin(inner, origin);
                Self { inner }
            }

            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: ::core::primitive::usize) -> Self {
                self.inner = self.inner.max_decoding_message_size(limit);
                self
            }

            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: ::core::primitive::usize) -> Self {
                self.inner = self.inner.max_encoding_message_size(limit);
                self
            }

            $(
                $crate::__define_client! {
                    $service_name,
                    $(#[$method_attr])*
                    $method,
                    $method_name,
                    ($($request)+),
                    ($($response)+)
                }
            )*

        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        ($request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::__private::tonic::IntoRequest<$request_ty>,
        ) -> $crate::__private::tonic::Result<$crate::__private::tonic::Response<$response_ty>> {
            $crate::__private::ready(&mut self.inner).await?;

            let mut req = $crate::__private::tonic::IntoRequest::into_request(request);
            req.extensions_mut()
                .insert($crate::__private::tonic::GrpcMethod::new($service_name, $method_name));

            let path = $crate::__private::http::uri::PathAndQuery::from_static(
                ::core::concat!("/", $service_name, "/", $method_name),
            );

            self.inner
                .unary(req, path, $crate::__private::tonic_prost::ProstCodec::new())
                .await
        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        ($request_ty:ty),
        (stream $response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::__private::tonic::IntoRequest<$request_ty>,
        ) -> $crate::__private::tonic::Result<
            $crate::__private::tonic::Response<$crate::__private::tonic::Streaming<$response_ty>>,
        > {
            $crate::__private::ready(&mut self.inner).await?;

            let mut req = $crate::__private::tonic::IntoRequest::into_request(request);
            req.extensions_mut()
                .insert($crate::__private::tonic::GrpcMethod::new($service_name, $method_name));

            let path = $crate::__private::http::uri::PathAndQuery::from_static(
                ::core::concat!("/", $service_name, "/", $method_name),
            );

            self.inner
                .server_streaming(req, path, $crate::__private::tonic_prost::ProstCodec::new())
                .await
        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        (stream $request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::__private::tonic::IntoStreamingRequest<Message = $request_ty>,
        ) -> $crate::__private::tonic::Result<$crate::__private::tonic::Response<$response_ty>> {
            $crate::__private::ready(&mut self.inner).await?;

            let mut req = $crate::__private::tonic::IntoStreamingRequest::into_streaming_request(request);
            req.extensions_mut()
                .insert($crate::__private::tonic::GrpcMethod::new($service_name, $method_name));

            let path = $crate::__private::http::uri::PathAndQuery::from_static(
                ::core::concat!("/", $service_name, "/", $method_name),
            );

            self.inner
                .client_streaming(req, path, $crate::__private::tonic_prost::ProstCodec::new())
                .await
        }
    };

    (
        $service_name:literal,
        $(#[$method_attr:meta])*
        $method:ident,
        $method_name:literal,
        (stream $request_ty:ty),
        (stream $response_ty:ty)
    ) => {
        $(#[$method_attr])*
        pub async fn $method(
            &mut self,
            request: impl $crate::__private::tonic::IntoStreamingRequest<Message = $request_ty>,
        ) -> $crate::__private::tonic::Result<
            $crate::__private::tonic::Response<$crate::__private::tonic::Streaming<$response_ty>>,
        > {
            $crate::__private::ready(&mut self.inner).await?;

            let mut req = $crate::__private::tonic::IntoStreamingRequest::into_streaming_request(request);
            req.extensions_mut()
                .insert($crate::__private::tonic::GrpcMethod::new($service_name, $method_name));

            let path = $crate::__private::http::uri::PathAndQuery::from_static(
                ::core::concat!("/", $service_name, "/", $method_name),
            );

            self.inner
                .streaming(req, path, $crate::__private::tonic_prost::ProstCodec::new())
                .await
        }
    };
}

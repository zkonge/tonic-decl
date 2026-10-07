#[doc(hidden)]
#[macro_export]
macro_rules! __define_server {
    (
        [$(#[$gate:meta])*]
        $(#[$server_attr:meta])*
        $trait_name:ident,
        $server_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ ) $(as $stream_name:ident)?;
        )*
    ) => {
        $(#[$gate])*
        $(#[$server_attr])*
        pub trait $trait_name: ::core::marker::Send + ::core::marker::Sync + 'static {
            $(
                $crate::__define_server! {
                    @trait_item
                    $(#[$method_attr])*
                    $method,
                    ($($request)+),
                    ($($response)+)
                    $(as $stream_name)?
                }
            )*
        }

        $crate::__define_server! {
            @server
            [$(#[$gate])*]
            $(#[$server_attr])*
            $server_name,
            $trait_name,
            $service_name,
            $(
                $(#[$method_attr])*
                fn $method($method_name)($($request)+) -> ($($response)+);
            )*
        }
    };

    (
        @server
        [$(#[$gate:meta])*]
        $(#[$server_attr:meta])*
        $server_name:ident,
        $trait_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ );
        )*
    ) => {
        $(#[$gate])*
        $(#[$server_attr])*
        #[derive(::core::fmt::Debug)]
        pub struct $server_name<T> {
            inner: ::std::sync::Arc<T>,
            max_decoding_message_size: ::core::option::Option<::core::primitive::usize>,
            max_encoding_message_size: ::core::option::Option<::core::primitive::usize>,
        }

        $(#[$gate])*
        impl<T: $trait_name> $server_name<T> {
            /// Creates a server that dispatches requests to `inner`.
            pub fn new(inner: T) -> Self {
                Self::from_arc(::std::sync::Arc::new(inner))
            }

            /// Creates a server that dispatches requests to a shared `inner`.
            pub fn from_arc(inner: ::std::sync::Arc<T>) -> Self {
                Self {
                    inner,
                    max_decoding_message_size: ::core::option::Option::None,
                    max_encoding_message_size: ::core::option::Option::None,
                }
            }

            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: ::core::primitive::usize) -> Self {
                self.max_decoding_message_size = ::core::option::Option::Some(limit);
                self
            }

            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: ::core::primitive::usize) -> Self {
                self.max_encoding_message_size = ::core::option::Option::Some(limit);
                self
            }

            #[inline]
            #[must_use]
            fn grpc<U, V>(
                &self,
            ) -> $crate::__private::tonic::server::Grpc<$crate::__private::tonic_prost::ProstCodec<U, V>>
            where
                U: $crate::__private::prost::Message + 'static,
                V: $crate::__private::prost::Message + ::core::default::Default + 'static,
            {
                $crate::__private::tonic::server::Grpc::new($crate::__private::tonic_prost::ProstCodec::new())
                    .apply_max_message_size_config(
                        self.max_decoding_message_size,
                        self.max_encoding_message_size,
                    )
            }

            /// Turns the server into a tower `Service` that routes requests by their path.
            ///
            /// The returned service can be passed to `tonic::transport::Server::add_service` or
            /// `tonic::service::Routes::new`, and wrapped with
            /// `tonic::service::interceptor::InterceptedService` to add an interceptor.
            #[must_use]
            pub fn into_service<B>(
                self,
            ) -> impl $crate::__private::tower_service::Service<
                $crate::__private::http::Request<B>,
                Response = $crate::__private::http::Response<$crate::__private::tonic::body::Body>,
                Error = ::core::convert::Infallible,
                Future = impl ::core::future::Future<
                    Output = ::core::result::Result<
                        $crate::__private::http::Response<$crate::__private::tonic::body::Body>,
                        ::core::convert::Infallible,
                    >,
                > + ::core::marker::Send,
            > + ::core::clone::Clone
            + $crate::__private::tonic::server::NamedService
            where
                B: $crate::__private::http_body::Body + ::core::marker::Send + 'static,
                B::Error: ::core::convert::Into<$crate::__private::StdError> + ::core::marker::Send + 'static,
            {
                // the trait methods may be deprecated by the caller
                #[allow(deprecated)]
                let svc = move |req: $crate::__private::http::Request<B>| {
                    let server = ::core::clone::Clone::clone(&self);
                    async move {
                        let inner = &*server.inner;

                        let resp = match req.uri().path().strip_prefix(::core::concat!("/", $service_name, "/")) {
                            $(
                                // method attributes are copied so that `cfg` removes the route
                                // together with the trait method, the others are inert here
                                #[allow(unused_attributes, unused_doc_comments)]
                                $(#[$method_attr])*
                                ::core::option::Option::Some($method_name) => {
                                    $crate::__define_server! {
                                        @route
                                        server,
                                        inner,
                                        req,
                                        $trait_name,
                                        $method,
                                        ($($request)+),
                                        ($($response)+)
                                    }
                                }
                            )*
                            _ => $crate::__private::grpc_unimplemented(),
                        };

                        ::core::result::Result::Ok(resp)
                    }
                };

                #[derive(::core::clone::Clone)]
                struct NamedServiceToken;

                impl $crate::__private::tonic::server::NamedService for NamedServiceToken {
                    const NAME: &'static ::core::primitive::str = $service_name;
                }

                $crate::__private::NamedSvcFn(svc, NamedServiceToken)
            }
        }

        $(#[$gate])*
        impl<T> ::core::clone::Clone for $server_name<T> {
            fn clone(&self) -> Self {
                Self {
                    inner: ::std::sync::Arc::clone(&self.inner),
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }
    };

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        ($request_ty:ty),
        ($response_ty:ty)
    ) => {{
        let s = $crate::__private::SvcFn(|req| $trait_name::$method($inner, req));
        $server.grpc().unary(s, $req).await
    }};

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        ($request_ty:ty),
        (stream $response_ty:ty)
    ) => {{
        let s = $crate::__private::SvcFn(|req| $trait_name::$method($inner, req));
        $server.grpc().server_streaming(s, $req).await
    }};

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        (stream $request_ty:ty),
        ($response_ty:ty)
    ) => {{
        let s = $crate::__private::SvcFn(|req| $trait_name::$method($inner, req));
        $server.grpc().client_streaming(s, $req).await
    }};

    (
        @route
        $server:ident,
        $inner:ident,
        $req:ident,
        $trait_name:ident,
        $method:ident,
        (stream $request_ty:ty),
        (stream $response_ty:ty)
    ) => {{
        let s = $crate::__private::SvcFn(|req| $trait_name::$method($inner, req));
        $server.grpc().streaming(s, $req).await
    }};

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        ($request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::__private::tonic::Request<$request_ty>,
        ) -> impl ::core::future::Future<
            Output = $crate::__private::tonic::Result<$crate::__private::tonic::Response<$response_ty>>,
        > + ::core::marker::Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        (stream $request_ty:ty),
        ($response_ty:ty)
    ) => {
        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::__private::tonic::Request<$crate::__private::tonic::Streaming<$request_ty>>,
        ) -> impl ::core::future::Future<
            Output = $crate::__private::tonic::Result<$crate::__private::tonic::Response<$response_ty>>,
        > + ::core::marker::Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        ($request_ty:ty),
        (stream $response_ty:ty)
    ) => {
        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::__private::tonic::Request<$request_ty>,
        ) -> impl ::core::future::Future<
            Output = $crate::__private::tonic::Result<
                $crate::__private::tonic::Response<
                    impl $crate::__private::futures_core::Stream<
                        Item = $crate::__private::tonic::Result<$response_ty>,
                    > + ::core::marker::Send + 'static,
                >,
            >,
        > + ::core::marker::Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        (stream $request_ty:ty),
        (stream $response_ty:ty)
    ) => {
        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::__private::tonic::Request<$crate::__private::tonic::Streaming<$request_ty>>,
        ) -> impl ::core::future::Future<
            Output = $crate::__private::tonic::Result<
                $crate::__private::tonic::Response<
                    impl $crate::__private::futures_core::Stream<
                        Item = $crate::__private::tonic::Result<$response_ty>,
                    > + ::core::marker::Send + 'static,
                >,
            >,
        > + ::core::marker::Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        ($request_ty:ty),
        (stream $response_ty:ty)
        as $stream_name:ident
    ) => {
        $(#[$method_attr])*
        type $stream_name: $crate::__private::futures_core::Stream<
            Item = $crate::__private::tonic::Result<$response_ty>,
        > + ::core::marker::Send;

        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::__private::tonic::Request<$request_ty>,
        ) -> impl ::core::future::Future<
            Output = $crate::__private::tonic::Result<$crate::__private::tonic::Response<Self::$stream_name>>,
        > + ::core::marker::Send;
    };

    (
        @trait_item
        $(#[$method_attr:meta])*
        $method:ident,
        (stream $request_ty:ty),
        (stream $response_ty:ty)
        as $stream_name:ident
    ) => {
        $(#[$method_attr])*
        type $stream_name: $crate::__private::futures_core::Stream<
            Item = $crate::__private::tonic::Result<$response_ty>,
        > + ::core::marker::Send;

        $(#[$method_attr])*
        fn $method(
            &self,
            req: $crate::__private::tonic::Request<$crate::__private::tonic::Streaming<$request_ty>>,
        ) -> impl ::core::future::Future<
            Output = $crate::__private::tonic::Result<$crate::__private::tonic::Response<Self::$stream_name>>,
        > + ::core::marker::Send;
    };
}

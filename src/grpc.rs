/// Declares the client and server stubs of a gRPC service.
///
/// ```text
/// define_grpc! {
///     /// Attributes here are applied to the client and server structs.
///     TraitName,
///     /// Attributes here are applied to all client items, e.g. `#[cfg(feature = "client")]`.
///     ClientName,
///     /// Attributes here are applied to all server items, the trait included.
///     ServerName,
///     "package.Service",
///
///     /// Attributes here are applied to the generated methods.
///     fn unary("Unary")(Request) -> (Response);
///     fn server_streaming("ServerStreaming")(Request) -> (stream Response);
///     fn client_streaming("ClientStreaming")(stream Request) -> (Response);
///     fn bidi_streaming("BidiStreaming")(stream Request) -> (stream Response) as BidiStreamingStream;
/// }
/// ```
///
/// Request and response types must implement [`prost::Message`].
///
/// A method returning a stream can name the stream type with `as`, which becomes an associated
/// type of the server trait. Without `as`, the trait method returns
/// `impl Stream<Item = tonic::Result<Response>> + Send + 'static`, so the implementation doesn't
/// have to spell out the stream type, e.g. a `map` over a closure.
///
/// It generates:
///
/// - `ClientName<T>`: a client over any [`GrpcService`](tonic::client::GrpcService) such as
///   `tonic::transport::Channel`.
/// - `TraitName`: the trait to implement on the server side, and `ServerName<T>`, which turns an
///   implementation into a tower `Service` with `into_service()`.
///
/// Both sides are generated unless gated. Attributes written before `ClientName` / `ServerName`
/// are copied to every item of that side, so `#[cfg(...)]` and `#[cfg_attr(...)]` make a side
/// optional, with whatever feature names the calling crate uses:
///
/// ```text
/// define_grpc! {
///     Greeter,
///     #[cfg(feature = "client")] GreeterClient,
///     #[cfg(feature = "server")] GreeterServer,
///     "helloworld.Greeter",
///     ...
/// }
/// ```
///
/// See the [crate documentation](crate) for a complete example.
#[macro_export]
macro_rules! define_grpc {
    (
        $(#[$grpc_attr:meta])*
        $trait_name:ident,
        $(#[$client_gate:meta])* $client_name:ident,
        $(#[$server_gate:meta])* $server_name:ident,
        $service_name:literal,
        $(
            $(#[$method_attr:meta])*
            fn $method:ident ($method_name:literal) ( $($request:tt)+ ) -> ( $($response:tt)+ ) $(as $stream_name:ident)?;
        )*
    ) => {
        $crate::__define_client! {
            [$(#[$client_gate])*]
            $(#[$grpc_attr])*
            $client_name,
            $service_name,
            $(
                $(#[$method_attr])*
                fn $method($method_name)($($request)+) -> ($($response)+);
            )*
        }

        $crate::__define_server! {
            [$(#[$server_gate])*]
            $(#[$grpc_attr])*
            $trait_name,
            $server_name,
            $service_name,
            $(
                $(#[$method_attr])*
                fn $method($method_name)($($request)+) -> ($($response)+) $(as $stream_name)?;
            )*
        }
    };
}

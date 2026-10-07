//! Tests for `tonic-decl`, kept in a separate crate to exercise the macros from another crate.
//!
//! This crate uses edition 2021 and expands `define_grpc!` in modules without the std prelude or
//! with std names shadowed, so the macros must not depend on the scope of the caller. The `echo`
//! module gates each side by a feature of this crate.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod pb {
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EchoRequest {
        #[prost(string, tag = "1")]
        pub message: String,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct EchoReply {
        #[prost(string, tag = "1")]
        pub message: String,
    }
}

/// Stubs expanded without the std prelude.
pub mod echo {
    #![no_implicit_prelude]

    #[cfg(any(feature = "client", feature = "server"))]
    use super::pb::{EchoReply, EchoRequest};

    ::tonic_decl::define_grpc! {
        /// Echoes messages back.
        Echo,
        #[cfg(feature = "client")] EchoClient,
        #[cfg(feature = "server")]
        #[cfg_attr(docsrs, doc(cfg(feature = "server")))]
        EchoServer,
        "test.Echo",

        /// Returns the request message.
        fn unary("Unary")(EchoRequest) -> (EchoReply);

        /// Returns the request message three times.
        fn server_streaming("ServerStreaming")(EchoRequest) -> (stream EchoReply);

        /// Returns the concatenation of all request messages.
        fn client_streaming("ClientStreaming")(stream EchoRequest) -> (EchoReply);

        /// Returns each request message.
        fn bidi_streaming("BidiStreaming")(stream EchoRequest) -> (stream EchoReply) as BidiStreamingStream;
    }

    // Same service, with a method the server doesn't implement.
    ::tonic_decl::define_grpc! {
        Missing,
        #[cfg(feature = "client")] MissingClient,
        #[cfg(feature = "server")] MissingServer,
        "test.Echo",

        fn missing("Missing")(EchoRequest) -> (EchoReply);
    }
}

/// Stubs with attributes on methods, checked with strict lints.
pub mod attrs {
    #![deny(missing_docs, deprecated)]

    use super::pb::{EchoReply, EchoRequest};

    ::tonic_decl::define_grpc! {
        /// Service with method attributes.
        Attrs,
        AttrsClient,
        AttrsServer,
        "test.Attrs",

        /// Named like the readiness check of the client.
        fn ready("Ready")(EchoRequest) -> (EchoReply);

        /// Always fails.
        #[deprecated(note = "use `ready`")]
        fn old("Old")(EchoRequest) -> (EchoReply);

        /// Never generated, nor routed.
        #[cfg(any())]
        fn removed("Removed")(EchoRequest) -> (EchoReply);

        /// Returns each request message, as an opaque stream.
        fn bidi("Bidi")(stream EchoRequest) -> (stream EchoReply);
    }
}

/// Stubs expanded next to items that shadow the std names used by the macros.
#[allow(dead_code, unused_imports, unused_macros, non_camel_case_types)]
pub mod shadowed {
    use ::std::io::Result;

    use super::pb::{EchoReply, EchoRequest};

    pub mod core {}
    pub mod std {}

    pub struct Option;
    pub struct Some;
    pub struct None;
    pub struct Ok;
    pub struct Err;
    pub struct Arc;
    pub struct Box;
    pub trait Future {}
    pub trait Clone {}
    pub trait Debug {}
    pub trait Default {}
    pub trait Into {}
    pub trait Send {}
    pub trait Sync {}
    pub type usize = u8;
    pub type str = u8;

    macro_rules! format {
        ($($t:tt)*) => {
            compile_error!("shadowed `format!` used")
        };
    }

    macro_rules! concat {
        ($($t:tt)*) => {
            compile_error!("shadowed `concat!` used")
        };
    }

    tonic_decl::define_grpc! {
        Echo,
        EchoClient,
        EchoServer,
        "test.Echo",

        fn unary("Unary")(EchoRequest) -> (EchoReply);
        fn server_streaming("ServerStreaming")(EchoRequest) -> (stream EchoReply) as ServerStreamingStream;
        fn server_streaming_opaque("ServerStreamingOpaque")(EchoRequest) -> (stream EchoReply);
        fn client_streaming("ClientStreaming")(stream EchoRequest) -> (EchoReply);
        fn bidi_streaming("BidiStreaming")(stream EchoRequest) -> (stream EchoReply) as BidiStreamingStream;
        fn bidi_streaming_opaque("BidiStreamingOpaque")(stream EchoRequest) -> (stream EchoReply);
    }
}

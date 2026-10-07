#![doc = include_str!("../README.md")]

mod client;
mod grpc;
mod server;
mod service;

/// Implementation details of the macros, not part of the public API.
#[doc(hidden)]
pub mod __private {
    pub use futures_core;
    pub use http;
    pub use http_body;
    pub use prost;
    pub use tonic;
    pub use tonic_prost;
    pub use tower_service;

    pub use crate::service::{NamedSvcFn, SvcFn, grpc_unimplemented, ready};

    pub type StdError = Box<dyn core::error::Error + Send + Sync + 'static>;
}

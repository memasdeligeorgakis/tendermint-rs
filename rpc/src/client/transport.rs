//! Tendermint RPC client implementations for different transports.

pub mod mock;
mod router;

#[cfg(any(feature = "http-client", feature = "http-client-web"))]
pub mod http;
#[cfg(feature = "websocket-client")]
pub mod websocket;

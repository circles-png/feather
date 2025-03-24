#![doc = include_str!("../README.md")]

/// The [`Middleware`] trait and some common middleware primitives.
pub mod middleware;
/// Synchronous API for Feather.
mod sync;

pub use crate::middleware::Middleware;
pub use crate::sync::AppConfig;
pub use sync::App;
pub use tiny_http::Request;
pub use tiny_http::Response;
pub use tiny_http::ResponseBox;

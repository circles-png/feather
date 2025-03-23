#![doc = include_str!("../README.md")]

/// The [`Middleware`] trait and some common middleware primitives.
pub mod middleware;
mod sync;
/// The [`Response`] type.
mod response;

pub use crate::middleware::Middleware;
pub use crate::sync::AppConfig;
pub use sync::App;
pub use tiny_http::Request;
pub use response::Response;

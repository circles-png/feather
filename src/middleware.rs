use std::str::FromStr;

use ascii::{AsciiString, FromAsciiError};
use dyn_clone::DynClone;
use tiny_http::{Header, HeaderField, Request, ResponseBox};

/// Common trait for all middleware types. Implemented automatically for functions fitting
/// the `(request, response) -> result` signature.
pub trait Middleware: Send + Sync + DynClone {
    /// Handle an incoming request by transforming it into a response.
    fn handle(
        &self,
        request: &mut Request,
        response: &mut ResponseBox,
    ) -> MiddlewareResult;
}

dyn_clone::clone_trait_object!(Middleware);

pub enum MiddlewareResult {
    /// Continue to the next middleware.
    Next,
    /// Skip all subsequent middleware and continue to the next route.
    NextRoute,
}

#[derive(Clone)]
/// Log incoming requests and transparently pass them to the next middleware.
pub struct Logger;

impl Middleware for Logger {
    fn handle(&self, request: &mut Request, _: &mut ResponseBox) -> MiddlewareResult {
        println!("Request: {request:?}");
        MiddlewareResult::Next
    }
}

#[derive(Clone, Default)]
/// Add [CORS] headers to the response.
///
/// [CORS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS
pub struct Cors(Option<AsciiString>);

impl Cors {
    /// Create a new middleware that adds CORS headers with the given origin.
    ///
    /// # Errors
    ///
    /// Returns an error if the given string is not valid ASCII.
    pub fn new(origin: String) -> Result<Self, FromAsciiError<String>> {
        AsciiString::from_ascii(origin).map(Some).map(Self)
    }
}

impl Middleware for Cors {
    fn handle(&self, _: &mut Request, response: &mut ResponseBox) -> MiddlewareResult {
        response.add_header(Header {
            field: HeaderField::from_str("Access-Control-Allow-Origin").unwrap(),
            value: self
                .0
                .clone()
                .unwrap_or_else(|| AsciiString::from_ascii("*").unwrap()),
        });
        MiddlewareResult::Next
    }
}

impl Middleware for [&Box<dyn Middleware>] {
    fn handle(
        &self,
        request: &mut Request,
        response: &mut ResponseBox,
    ) -> MiddlewareResult {
        for middleware in self {
            if matches!(
                middleware.handle(request, response),
                MiddlewareResult::NextRoute
            ) {
                return MiddlewareResult::NextRoute;
            }
        }
        MiddlewareResult::Next
    }
}

impl<F: Fn(&mut Request, &mut ResponseBox) -> MiddlewareResult + Sync + Send + DynClone>
    Middleware for F
{
    fn handle(
        &self,
        request: &mut Request,
        response: &mut ResponseBox,
    ) -> MiddlewareResult {
        self(request, response)
    }
}

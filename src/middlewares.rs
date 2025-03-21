use crate::types::Response;
use crate::sync::Next;
use tiny_http::Request;

pub trait Middleware: Send + Sync + dyn_clone::DynClone {
    fn handle(self:&Self, req: &mut Request, next: Next) -> Response;
}

dyn_clone::clone_trait_object!(Middleware);

#[derive(Clone)]
pub struct Logger;

impl Middleware for Logger {
    fn handle(self:&Self, req: &mut Request, next: Next) -> Response {
        println!("Request: {:?}", req);
        next(req)
    }
}

#[derive(Clone)]
pub struct Cors(Option<String>);

impl Cors {
    pub fn new(origin: &str) -> Self {
        Self(Some(origin.to_string()))
    }
}

impl Default for Cors {
    fn default() -> Self {
        Self(None)
    }
}

impl Middleware for Cors {
    fn handle(self:&Self, req: &mut Request, next: Next,) -> Response {
        
        let res = next(req);
        match self.0 {
            Some(ref origin) => {
                res.with_header("Access-Control-Allow-Origin", origin)
            }
            None => {
                res.with_header("Access-Control-Allow-Origin", "*")
            }
        }
    }
}

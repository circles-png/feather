use crate::types::Response;
use crate::sync::Middleware;
use std::fs::File;
use std::sync::Arc;
use serde_json::Value;

pub enum Middlewares {
    Logger,
    ParseJson,
    ServeStatic { dir: String },
}

impl Middlewares {
    pub(crate) fn into_middleware(self) -> Middleware {
        match self {
            Middlewares::Logger => Arc::new(|req, next| {
                println!("{} {} - {:?}", req.method(), req.url(), req.remote_addr());
                next(req)
            }),
            Middlewares::ParseJson => Arc::new(|req, next| {
                //let mut body = String::new();
                let mut body = String::new();
                req.as_reader().read_to_string(&mut body).unwrap_or(0);
                if let Ok(json) = serde_json::from_str::<Value>(&body){
                    println!("Parsed Data: {}", json)
                }
                next(req) // Orijinal isteği bir sonraki middleware'e ilet
            }),
            Middlewares::ServeStatic { dir } => Arc::new(move |req, next| {
                let path = format!("{}/{}", dir, req.url().trim_start_matches('/'));
                let path = std::path::Path::new(&path);

                if path.is_file() {
                    if let Ok(contents) = File::open(path) {
                        Response::from_file(200, contents)
                    } else {
                        next(req)
                    }
                } else {
                    next(req)
                }
            }),
        }
    }
}

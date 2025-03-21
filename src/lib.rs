//! # Feather Framework
//!
//! Feather is a lightweight and flexible web framework inspired from Express.js, designed to be easy to use and extensible.
//! It provides a simple API for defining routes, handling requests, and managing middleware.
//!
//! ## Features
//!
//! - Simple and intuitive API for defining routes and handlers
//! - Support for middleware to process requests and responses
//! - Configurable thread pool for handling concurrent requests
//! - Built on top of `tiny_http` for efficient HTTP handling
//!
//! ## Example
//!
//! ```rust,no_run
//! //* Import Dependencies from Feather
//! use feather::{App, AppConfig};
//! use feather::Response;
//! use feather::middlewares::Logger;
//! //* Main Function No Async Here
//! fn main() {
//!    //* Create instance of AppConfig with 4 threads
//!     let config = AppConfig { threads: 4 };
//!      //* Create a new instance of App
//!      let mut app = App::new(config);
//!      //*Define a route for the root path
//!      app.get("/", |_req| {
//!          Response::ok("Hello From Feather")
//!      });    
//!      //* Use the Logger middleware
//!      app.use_middleware(Logger);
//!      //* Listen on port 3000
//!      app.listen("127.0.0.1:3000");
//!  }
//! ```
//! ## Using and making your own Middleware
//!
//! ```rust,no_run
//! //* Import Dependencies from Feather
//! use feather::{App, AppConfig, Next, Request};
//! use feather::Response;
//! //* Import the Needed Trait and Structs from the Middlewares Module
//! use feather::middlewares::{Logger,Middleware};
//! fn main(){
//!     //* Define an app
//!     let config = AppConfig { threads: 4 };
//!     let mut app = App::new(config);
//!     //* Use a builtin Middleware
//!     app.use_middleware(Logger);
//!     //* Define a route
//!     app.get("/", |_req| {
//!         Response::ok("Hello From Feather!!")
//!     });
//!     //* Listen on port 3000
//!     app.listen("127.0.0.1:3000");
//! }
//! //* Creating Our Own Middleware
//! // Middlewares are structs that implement the Middleware trait.
//! #[derive(Clone)]
//! struct MyMiddleman;

//! // The Middleware trait defines a single method, handle, which takes a mutable reference to a Request and a Next function.
//! impl Middleware for MyMiddleman {
//!     fn handle(self:&Self, req: &mut Request, next: Next) -> Response {
//!         // Do Stuff Here
//!         print!("My Middleman is Here!!");
//!         print!("And There is a Request From: {:?}", req.url());
//!         // and then call the next middleware in the chain
//!         next(req)
//!     }  
//! }
//! ```
//! ## Modules
//!
//! - `middlewares`: Contains the middleware trait and related functionality
//! - `types`: Defines common types used throughout the framework
//! - `sync`: Contains the main application and configuration structs
//!
//! ## Re-exports
//!
//! The following items are re-exported for convenience:
//!
//! - `AppConfig`: Configuration settings for the application
//! - `Middleware`: Trait for defining middleware
//! - `Response`: Type for HTTP responses
//! - `Request`: Type for HTTP requests
//! - `App`: The main application struct
//! - `Next`: Type alias for the next middleware function

pub mod middlewares;
mod types;
mod sync;

pub use crate::sync::AppConfig;
pub use crate::middlewares::Middleware;
pub use types::Response;
pub use tiny_http::Request;
pub use sync::App;
pub use sync::Next;
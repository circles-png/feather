# Feather

Feather is a lightweight, flexible, and highly extensible web framework for Rust, inspired by the simplicity and modularity of Express.js.

---

## Features

- **Middleware Support**: Chain and compose middleware functions effortlessly.
- **Route Handling**: Support for multiple HTTP methods (`GET`, `POST`, `PUT`, `DELETE`, etc.).
- **Lightweight**: Built using Rust’s high-performance and memory-safe features.
- **Thread-Safe**: Leverages Rust's thread safe aspects.

## Modules

- `middleware`: Contains the middleware trait and related functionality
- `response`: Defines the Response type used throughout the framework
- `sync`: Contains the main application and configuration structs

## Re-exports

The following items are re-exported for convenience:

- `AppConfig`: Configuration settings for the application
- `Middleware`: Trait for defining middleware
- `Response`: Type for HTTP responses
- `Request`: Type for HTTP requests
- `App`: The main application struct
- `Next`: Type alias for the next middleware function

---

## Installation

To get started with Feather, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
feather = "0.1.1"
```

## Quick Start

Here's an example of building a simple web server with Feather:

```rust,no_run
// Import dependencies from Feather
use feather::middleware::{Logger, MiddlewareResult};
use feather::{App, AppConfig, Response};

// Main function - no async here!
// Create instance of AppConfig with 4 threads
let config = AppConfig { threads: 4 };

// Create a new instance of App
let mut app = App::new(config);

// Define a route for the root path
app.get("/", |_request: &mut _, response: &mut _| {
    *response = Response::from_string("Hello from Feather!").boxed();
    MiddlewareResult::Next
});
// Use the Logger middleware for all routes
app.use_middleware(Logger);
// Listen on port 3000
app.listen("127.0.0.1:3000");
```

---

## Middleware

Feather supports middleware for pre-processing requests and post-processing responses, and you can make your own too! Here's an example:

```rust,no_run
// Import dependencies from Feather
use feather::{App, AppConfig, Request, Response, ResponseBox};
// Import the Middleware trait and some common middleware primitives
use feather::middleware::{Logger, Middleware, MiddlewareResult};

// Implementors of the Middleware trait are middleware that can be used in a Feather app.
#[derive(Clone)]
struct Custom;

// The Middleware trait defines a single method `handle`,
// which can mutate the request and response objects, then return a `MiddlewareResult`.
impl Middleware for Custom {
    fn handle(&self, request: &mut Request, _response: &mut ResponseBox) -> MiddlewareResult {
        // Do stuff here
        println!("Now running some custom middleware (struct Custom)!");
        println!("And there's a request with path: {:?}", request.url());
        // and then continue to the next middleware in the chain
        MiddlewareResult::Next
    }
}

// Define an app
let config = AppConfig { threads: 4 };
let mut app = App::new(config);

// Use the builtin Logger middleware for all routes
app.use_middleware(Logger);

// Use the Custom middleware for all routes
app.use_middleware(Custom);

// Use another middleware defined by a function for all routes
app.use_middleware(|_request: &mut _, _response: &mut _| {
    println!("Now running some custom middleware (closure)!");
    MiddlewareResult::Next
});

// Define a route
app.get("/", |_request: &mut _, response: &mut _| {
    *response = Response::from_string("Hello from Feather!").boxed();
    MiddlewareResult::Next
});

// Listen on port 3000
app.listen("127.0.0.1:3000");
```

Built-in middleware includes:

- `Logger`: Logs incoming requests.
- `Cors`: Add cross-origin resource sharing headers to your response.

---

## Goals

- Be the simple & beginner-friendly web framework for Rust
- Be modular and expandable by design
- Be easy to use and learn

## Non-Goals

- Be the most powerful/performant web framework
- Use complex and low-level features
- Be the most feature-rich web framework

---

## Contributing

Contributions are welcome! If you have ideas for improving Feather or find a bug, feel free to open an issue or submit a pull request.

1. Fork the repository.
2. Create your feature branch: `git checkout -b feature/my-feature`.
3. Commit your changes: `git commit -m 'Add my feature'`.
4. Push to the branch: `git push origin feature/my-feature`.
5. Open a pull request.

---

## License

Feather is open-source software, licensed under the [MIT License](LICENSE).

---

## Acknowledgments

Feather is inspired by the simplicity of Express.js and aims to bring similar productivity to the Rust ecosystem. Special thanks to the Rust community for their contributions to building robust tools and libraries.

---

# Feather

Feather is a lightweight, flexible, and highly extensible web framework for Rust, inspired by the simplicity and modularity of Express.js.

---

## Features

- **Middleware Support**: Chain and compose middleware functions effortlessly.
- **Route Handling**: Support for multiple HTTP methods (`GET`, `POST`, `PUT`, `DELETE`, etc.).
- **Lightweight**: Built using Rust’s high-performance and memory-safe features.
- **Thread-Safe**: Leverages Rust's Thread Safe Aspects.  
---

## Installation

To get started with Feather, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
feather = "0.0.1"
```

## Quick Start

Here's an example of building a simple web server with Feather:

```rust
use feather::types::Response;
use feather::sync::App;

fn main() {
    let mut app = App::new();

    // Add a GET route
    app.get("/", |req| {
        Response::ok("Welcome to Feather!")
    });

    // Add a POST route
    app.post("/data", |req| {
        let mut body = String::new();
        if let Ok(res) = req.as_reader().read_to_string(&mut body){
            Response::ok(format!("Received Body: {}", res))
        }else {
            Response::new(400, "Bad Request")
        }
    });

    // Listen on localhost:8080
    app.listen("127.0.0.1:8080");
}
```

---

## Middleware

Feather supports middleware for pre-processing requests and post-processing responses. Here's an example:

```rust
use feather::sync::App;

fn main() {
    let mut app = App::new();

    // Add a logging middleware
    app.use_middleware(Middleware::new(|req, next| {
        println!("Incoming request: {} {}", req.method(), req.url());
        next(req)
    }));

    app.get("/", |req| Response::ok("Hello, Feather!"));

    app.listen("127.0.0.1:8080");
}
```

Built-in middleware includes:
- `logger`: Logs incoming Requests
- `parse_json`: Parses incoming JSON requests.
- `serve_static`(WIP): Serves static files from a specified directory.

---
## Goals
- Be the simplest Web Framework for Rust
- Support Sync and Async Programing Paradigms

## Non-Goals
- Be the most powerful/performant Web Framework
- Use Complex and Low-level features

---
## Planned Features
- Full Featured async support with Tokio
- More builtin middlewares
- Ctrl-C Gracefull Shutdown
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


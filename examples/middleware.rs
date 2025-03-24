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

fn main() {
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
}

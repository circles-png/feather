//*Import Dependencies from Feather
use feather::Response;
use feather::{App, AppConfig, Request};
//*Import the Needed Trait and Structs from the Middlewares Module
use feather::middleware::{Logger, Middleware, MiddlewareResult};

fn main() {
    //*Define an app
    let config = AppConfig { threads: 4 };
    let mut app = App::new(config);

    //* Use a builtin Middleware
    app.use_middleware(Logger);

    //* Define a route
    app.get("/", |_request: &mut _, _response: &mut _| {
        Response::ok("Hello From Feather!!");
        MiddlewareResult::Next
    });

    //* Listen on port 3000
    app.listen("127.0.0.1:3000");
}

//* Creating Our Own Middleware
// Middleware are structs that implement the Middleware trait.
#[derive(Clone)]
struct MyMiddleman;

// The Middleware trait defines a single method, handle, which takes a mutable reference to a Request and a Next function.
impl Middleware for MyMiddleman {
    fn handle(&self, request: &mut Request, _response: &mut Response) -> MiddlewareResult {
        // Do Stuff Here
        print!("My Middleman is Here!!");
        print!("And There is a Request From: {:?}", request.url());
        // and then call the next middleware in the chain
        MiddlewareResult::Next
    }
}

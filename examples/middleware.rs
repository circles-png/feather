//*Import Dependencies from Feather
use feather::{App, AppConfig, Next, Request};
use feather::Response;
//*Import the Needed Trait and Structs from the Middlewares Module
use feather::middlewares::{Logger,Middleware};

fn main(){
    //*Define an app
    let config = AppConfig { threads: 4 };
    let mut app = App::new(config);

    //* Use a builtin Middleware
    app.use_middleware(Logger);

    //* Define a route
    app.get("/", |_req| {
        Response::ok("Hello From Feather!!")
    });

    //* Listen on port 3000
    app.listen("127.0.0.1:3000");
}

//* Creating Our Own Middleware
// Middlewares are structs that implement the Middleware trait.
#[derive(Clone)]
struct MyMiddleman;

// The Middleware trait defines a single method, handle, which takes a mutable reference to a Request and a Next function.
impl Middleware for MyMiddleman {
    fn handle(self:&Self, req: &mut Request, next: Next) -> Response {
        // Do Stuff Here
        print!("My Middleman is Here!!");
        print!("And There is a Request From: {:?}", req.url());
        // and then call the next middleware in the chain
        next(req)
    }
    
}
//*Import Dependencies from Feather
use feather::{App, AppConfig};
use feather::Response;
use feather::middlewares::Logger;
//*Main Function No Async Here
fn main() {
    //*Create instance of AppConfig with 4 threads
    let config = AppConfig { threads: 4 };

    //*Create a new instance of App
    let mut app = App::new(config);

    //*Define a route for the root path
    app.get("/", |_req| {
        Response::ok("Hello From Feather")
    });    
    //*Use the Logger middleware
    app.use_middleware(Logger);
    //*Listen on port 3000
    app.listen("127.0.0.1:3000");
}


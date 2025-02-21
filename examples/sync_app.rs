use feather::sync::App;
use feather::types::Response;
use feather::middlewares::Middlewares;

fn main() {
    let mut app = App::new();
    app.use_builtin(Middlewares::Logger);
    app.get("/", |res| Response::ok("Eyo"));    

    app.listen("127.0.0.1:3000");
}
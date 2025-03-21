use reqwest::blocking::get;
use feather::{App, AppConfig};
use feather::Response;
use std::thread;

#[test]
fn test_app() {
    let config = AppConfig { threads: 4 };
    let mut app = App::new(config);

    app.get("/", |_req| {
        Response::ok("Hello From Feather")
    });
    thread::spawn(move || {
        app.listen("127.0.0.1:3000");
    });

    let resp = get("http://127.0.0.1:3000").unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.text().unwrap(), "Hello From Feather");
}
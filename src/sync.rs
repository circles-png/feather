use crate::middlewares::Middleware;
use crate::types::Response;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use threadpool::ThreadPool;
use tiny_http::{Request, Server};

pub type Next = Arc<dyn Fn(&mut Request) -> Response + Send + Sync>;


#[derive(Clone)]
struct CloneableFn(Arc<dyn for<'a> Fn(&mut tiny_http::Request) -> Response + Send + Sync>);

impl CloneableFn {
    pub fn new<F>(f: F) -> Self
    where
        F: for<'a> Fn(&mut tiny_http::Request) -> Response + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }

    pub fn call(&self, req: &mut tiny_http::Request) -> Response {
        (self.0)(req)
    }
}

/// Configuration settings for the application.
///
/// This struct is used to configure various aspects of the application,
/// such as the number of threads to be used in the thread pool.
///
/// # Fields
///
/// * `threads` - The number of threads to be used by the application's thread pool.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub threads: usize,
}

pub struct App {
    config: AppConfig,
    routes: Arc<RwLock<HashMap<(String, String), CloneableFn>>>,
    middlewares: Arc<Mutex<Vec<Box<dyn Middleware>>>>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            routes: Arc::new(RwLock::new(HashMap::new())),
            middlewares: Arc::new(Mutex::new(Vec::new())),
        }
    }
    #[inline]
    pub fn use_middleware<MD: Middleware + 'static>(&mut self, middleware: MD ) {
        self.middlewares.lock().unwrap().push(Box::new(middleware));
    }
    #[inline]
    pub fn route<H>(&mut self, medhod: &str, path: &str, handler: H)
    where
        H: Fn(&mut Request) -> Response + 'static + Send + Sync,
    {
        let key = (medhod.to_string(), path.to_string());
        self.routes
            .write()
            .unwrap()
            .insert(key, CloneableFn::new(handler));
    }
    #[inline]
    pub fn get<H>(&mut self, path: &str, handler: H)
    where
        H: Fn(&mut Request) -> Response + 'static + Send + Sync,
    {
        self.route("GET", path, handler);
    }
    #[inline]
    pub fn post<H>(&mut self, path: &str, handler: H)
    where
        H: Fn(&mut Request) -> Response + 'static + Send + Sync,
    {
        self.route("POST", path, handler);
    }
    #[inline]
    pub fn delete<H>(&mut self, path: &str, handler: H)
    where
        H: Fn(&mut Request) -> Response + 'static + Send + Sync,
    {
        self.route("DELETE", path, handler);
    }
    #[inline]
    pub fn put<H>(&mut self, path: &str, handler: H)
    where
        H: Fn(&mut Request) -> Response + 'static + Send + Sync,
    {
        self.route("PUT", path, handler);
    }
    #[inline]
    pub fn listen(&self, address: &str) {
        let server = Arc::new(Server::http(address).expect("Failed to start server"));
        println!("Listening on http://{}", address);
        let pool = ThreadPool::new(self.config.threads);    

        for mut rq in server.incoming_requests() {
            let path = rq.url().to_string();
            let method = rq.method().to_string();
            let routes = Arc::clone(&self.routes);
            let middlewares = Arc::clone(&self.middlewares);
            
            pool.execute(move || {
                
                // Handler'ı al
                let handler = {
                    let routes_guard = routes.read().unwrap();
                    routes_guard.get(&(method.clone(), path.clone())).cloned()
                };

                // İlk final handler'ı oluştur
                let mut next: Next = Arc::new(move |req: &mut Request| -> Response {
                    if let Some(handler) = &handler {
                        handler.call(req)
                    } else {
                        Response::ok("404 Not Found")
                    }
                });

                // Middleware'leri ters sırada çalıştır
                let middlewares_cloned: Vec<_> = {
                    let middlewares_guard = middlewares.lock().unwrap();
                    middlewares_guard.clone()
                };

                for middleware in middlewares_cloned.iter().rev() {
                    let current_middleware = middleware.clone();
                    let current_next = next.clone();

                    next = Arc::new(move |req: &mut Request| -> Response {
                        current_middleware.handle(req, current_next.clone())
                    });
                }

                // Zincirin ilk middleware'ini çağır
                let response = next(&mut rq);
                

                // Yanıtı gönder
                if response.is_file() {
                    if let Err(e) = rq.respond(response.into_tiny_http_file()) {
                        eprintln!("Response Failed To Send: {e}");
                    }
                } else {
                    if let Err(e) = rq.respond(response.into_tiny_http_cursor()) {
                        eprintln!("Response Failed To Send: {e}");
                    }
                }
            });

            pool.join();
        }
    }
}

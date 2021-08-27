use super::{Request, Response};
use std::collections::HashMap;

pub type Callback = fn(&Request) -> Response;
#[derive(Clone)]
pub struct Router {
    handlers: HashMap<String, Handler>,
}

#[derive(Clone)]
enum Handler {
    Callback(Callback),
    File(String),
}

impl Router {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    pub fn handle_request(&self, req: &Request) -> Response {
        let handler = match self.handlers.iter().find(|(k, _)| req.path == **k) {
            Some((_, v)) => v,
            None => return Response::not_found(),
        };
        match handler {
            Handler::Callback(cb) => cb(req),
            Handler::File(fname) => Response::file(fname),
        }
    }
    pub fn insert_callback(&mut self, pat: &str, cb: Callback) {
        self.handlers.insert(pat.to_string(), Handler::Callback(cb));
    }
    pub fn insert_file(&mut self, pat: &str, fname: &str) {
        self.handlers
            .insert(pat.to_string(), Handler::File(fname.to_string()));
    }
}

use std::{collections::HashMap, fs::File, io::Cursor};



pub struct Response {
    status_code: u16,
    body: Option<String>,
    file: Option<File>,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new(status_code: u16, body: impl Into<String>) -> Self {
        Self {
            status_code,
            body: Some(body.into()),
            file: None,
            headers: HashMap::new(),
        }
    }
    pub fn ok(body: impl Into<String>) -> Self {
        Self::new(200, body)
    }

    pub fn from_file(status_code: u16, file: File) -> Self {
        Self {
            status_code,
            body: None,
            file: Some(file),
            headers: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub(crate) fn is_file(&self) -> bool {
        match &self.file {
            None => {
                return false;
            }
            Some(_) => {
                return true;
            }
        }
    }

    pub(crate) fn into_tiny_http_cursor(self) -> tiny_http::Response<Cursor<Vec<u8>>> {
        let mut response =
            tiny_http::Response::from_string(self.body.unwrap()).with_status_code(self.status_code);

        for (key, value) in self.headers {
            response.add_header(
                tiny_http::Header::from_bytes(key.as_bytes(), value.as_bytes()).unwrap(),
            );
        }
        response
    }

    pub(crate) fn into_tiny_http_file(self) -> tiny_http::Response<File> {
        let mut response =
            tiny_http::Response::from_file(self.file.unwrap()).with_status_code(self.status_code);

        for (key, value) in self.headers {
            response.add_header(
                tiny_http::Header::from_bytes(key.as_bytes(), value.as_bytes()).unwrap(),
            );
        }
        response
    }
}

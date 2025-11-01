use std::collections::HashMap;

pub enum HttpStatus {
    Ok,
    Created,
    NotFound,
}

pub enum Body {
    Text(String),
    Binary(Vec<u8>),
}

pub struct Response {
    status_code: HttpStatus,
    body: Option<Body>,
    headers: Option<HashMap<String, String>>,
}

impl Response {
    pub fn new(status_code: HttpStatus) -> Self {
        Self {
            status_code,
            body: None,
            headers: None,
        }
    }

    pub fn set_body(mut self, body: String) -> Self {
        self.body = Some(Body::Text(body));
        self
    }

    pub fn set_body_bytes(mut self, body: Vec<u8>) -> Self {
        self.body = Some(Body::Binary(body));
        self
    }

    pub fn add_header(mut self, key: String, value: String) -> Self {
        if self.headers.is_some() {
            self.headers.as_mut().unwrap().insert(key, value);
            self
        } else {
            let mut new_headers = HashMap::new();
            new_headers.insert(key, value);
            self.headers = Some(new_headers);
            self
        }
    }

    pub fn into_response(self) -> Vec<u8> {
        let mut response = Vec::new();
        // Status line
        let status_line = match self.status_code {
            HttpStatus::Ok => "HTTP/1.1 200 OK\r\n",
            HttpStatus::Created => "HTTP/1.1 201 Created\r\n",
            HttpStatus::NotFound => "HTTP/1.1 404 Not Found\r\n",
        };
        response.extend_from_slice(status_line.as_bytes());

        // Headers
        if let Some(headers) = self.headers {
            for (k, v) in headers {
                response.extend_from_slice(format!("{}: {}\r\n", k, v).as_bytes());
            }
        }

        response.extend_from_slice(b"\r\n");

        // Body
        if let Some(body) = self.body {
            match body {
                Body::Text(text) => {
                    response.extend_from_slice(text.as_bytes());
                }
                Body::Binary(bytes) => {
                    response.extend_from_slice(&bytes);
                }
            }
        }
        response
    }
}

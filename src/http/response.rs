use std::collections::HashMap;

pub enum HttpStatus {
    Ok,
    Created,
    NotFound,
}
pub struct Response {
    status_code: HttpStatus,
    body: Option<String>,
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
        self.body = Some(body);
        return self;
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
        let mut response_str = String::new();
        match self.status_code {
            HttpStatus::Ok => {
                response_str.push_str("HTTP/1.1 200 OK\r\n");
            }
            HttpStatus::Created => {
                response_str.push_str("HTTP/1.1 201 Created\r\n");
            }
            HttpStatus::NotFound => {
                response_str.push_str("HTTP/1.1 404 Not Found\r\n");
            }
        }

        match self.headers {
            Some(headers) => {
                for (k, v) in headers {
                    response_str.push_str(format!("{}: {}\r\n", k, v).as_str());
                }
                response_str.push_str("\r\n");
            }
            None => {
                response_str.push_str("\r\n");
            }
        }

        match self.body {
            Some(body) => {
                response_str.push_str(format!("{}\r\n", body).as_str());
            }
            None => {}
        }
        response_str.push_str("\r\n");
        response_str.into_bytes()
    }
}

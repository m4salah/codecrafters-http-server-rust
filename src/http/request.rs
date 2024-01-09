use std::collections::HashMap;

use super::methods::Method;

// Request object that hold all information about the request
#[derive(Debug, Clone)]
pub struct Request {
    pub path: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

// Impl the from method to try to construct the the Request object from parsing the raw string
impl From<String> for Request {
    fn from(value: String) -> Self {
        // split the value to sections
        let mut sections = value.split("\r\n\r\n");

        // the first sections containing the meta data of the http like the method, path and headers
        // and splitted into lines
        let mut meta_section = sections.next().unwrap().lines();

        // get the first line that contains the method, http version, and path
        let method_path = meta_section.next().unwrap();
        let mut splitted_method_path = method_path.split(' ');

        let method: Method = splitted_method_path.next().unwrap().try_into().unwrap();
        let path = splitted_method_path.next().unwrap();

        let headers: HashMap<String, String> = meta_section
            .filter_map(|line| {
                return line
                    .split_once(": ")
                    .map(|s| (s.0.to_string(), s.1.to_string()));
            })
            .collect();

        // it's the next on sections but i want it to be String
        let body = match sections.next() {
            Some(body) => {
                if body.is_empty() {
                    None
                } else {
                    Some(body.to_string())
                }
            }
            None => None,
        };
        return Request {
            path: path.to_string(),
            method,
            headers,
            body,
        };
    }
}

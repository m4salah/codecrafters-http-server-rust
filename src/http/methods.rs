// Method enum that hold all supported methods
#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    Get,
    Post,
}

// try to construct the Method enum from string
impl TryFrom<&str> for Method {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "POST" => Ok(Method::Post),
            "GET" => Ok(Method::Get),
            _ => Err("invalid method".to_string()),
        }
    }
}

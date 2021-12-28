use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buffer: &[u8]) -> Result<Self, String> {}
}

impl TryFrom for Request {}

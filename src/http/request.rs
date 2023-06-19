use super::method::Method; //Relative path

pub struct Request {
    path: String,
    query_string: Option<String>, //Rust doesn't support "No value" - use Option enum
    method: Method,
}            
use super::method::Method; //Relative path
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter};
use std::str;
use std::str::Utf8Error;
pub struct Request {
    path: String,
    query_string: Option<String>, //Rust doesn't support "No value" - use Option enum
    method: Method,
}            

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1 
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }
        //
        let request = str::from_utf8(buf)?;
        unimplemented!()
    }
}


fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i+1..])); //1 is 1byte, ' ' is 1 byte so this works safely
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self)-> &str{
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol=>"InvalidProtocol",
            Self::InvalidMethod=>"InvalidMethod",  
        }
    }
}

impl Error for ParseError {

}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         write!(f, "{}", self.message())
    }
}
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
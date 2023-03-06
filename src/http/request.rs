use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // request: GET / HTTP/1.1
    // Host: 127.0.0.1:8080
    // User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/110.0
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
    // Accept-Language: en-US,en;q=0.5
    // Accept-Encoding: gzip, deflate, br
    // DNT: 1
    // Connection: keep-alive
    // Upgrade-Insecure-Requests: 1
    // Sec-Fetch-Dest: document
    // Sec-Fetch-Mode: navigate
    // Sec-Fetch-Site: none
    // Sec-Fetch-User: ?1
    // Sec-GPC: 1

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "invalid request",
            Self::InvalidEncoding => "invalid encoding",
            Self::InvalidProtocol => "invalid protocol",
            Self::InvalidMethod => "invalid method",
        }
    }
}

impl Error for ParseError {}

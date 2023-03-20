use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
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

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // match path.find('?') {
        //     Some(index) => {
        //         query_string = Some(&path[index + 1..]);
        //         path = &path[..index];
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let index = q.unwrap();
        //     query_string = Some(&path[index + 1..]);
        //     path = &path[..index];
        // }
        if let Some(index) = path.find('?') {
            query_string = Some(&path[index + 1..]);
            path = &path[..index];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..index], &request[index + 1..]));
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

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
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

use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Result as FmtResult};
use std::str::{self, Utf8Error};

/*
GET /usr?id=10 HTTP/1.1 \r\n
HEADERS \r\n <- igonore
BODY <- ignore
*/

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// impl Request {
//     pub fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let req = str::from_utf8(&buf)?;
        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        // 'GET' のような文字列から Method Enum へ型変換
        let method = method.parse::<Method>()?;

        // path から クエリ文字列を取得する
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..])); // "?" も1byte
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // i + 1  この1バイト足す行為はほんまは危ないけど、' 'スペースは必ず1バイトだとわかっているからOK
            return Some((&req[..i], &req[i + 1..]));
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

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

// let req = str::from_utf8(&buf)?; だけでかけるようにする!
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

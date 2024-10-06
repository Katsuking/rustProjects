use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    PUT,
    POST,
    PATCH,
    CONNECT,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PUT" => Ok(Self::PUT),
            "POST" => Ok(Self::POST),
            "PATCH" => Ok(Self::PATCH),
            "CONNECT" => Ok(Self::CONNECT),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;

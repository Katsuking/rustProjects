use std::io::{Result as ioResult, Write};
use std::net::TcpStream;

use super::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    // tcp stream にデータを書き込む
    pub fn send(&self, stream: &mut impl Write) -> ioResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

// ただまとめて、ヒープに保存するだけならこれでいいけど、
// 20000行とかのHTML を対処できん
// impl Display for Response {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let body = match &self.body {
//             Some(b) => b,
//             None => "",
//         };

//         write!(
//             f,
//             "HTTP/1.1 {} {}\r\n\r\n{}",
//             self.status_code,
//             self.status_code.reason_phrase(),
//             body
//         )
//     }
// }

use crate::http::{ParseError, Request, Response, StatusCode};
use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub struct Server {
    addr: String,
}

// Response::new()でインスタンスを作るのではなく、
// Handlerを作って、Responseをさばく
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        // this is the default implementaion
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    // associated func
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    // method
    pub fn run(self, mut handler: impl Handler) {
        println!("running on: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            // &buf[..]
                            let res = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = res.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish the connection: {}", e),
            }
        }
    }
}

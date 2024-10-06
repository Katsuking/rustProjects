use crate::http::request::Request;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub struct Server {
    addr: String,
}

impl Server {
    // associated func
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    // method
    pub fn run(self) {
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
                            match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 404 NotFound\r\n");
                                }
                                Err(e) => println!("Error!: {}", e),
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

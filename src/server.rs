use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        // syntax sugar for infinate loop
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            // :? use the format of debug
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let resp = match Request::try_from(&buf[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = resp.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(msg) => println!("Connection failed: {}", msg),
            }
        }
    }
}

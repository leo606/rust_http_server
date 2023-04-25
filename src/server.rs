use crate::http::{Request, Response, StatusCode, ParseError};
use std::io::Read;
use std::convert::TryFrom;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.address).unwrap();

        println!("running!!! on {}", self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    handler.handle_request(&req)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response {}", e);
                            }

                        }
                        Err(error) => {
                            println!("some error on read stream {}", error);
                        }
                    }
                }
                Err(_) => {
                    println!("error")
                }
            }
        }
    }
}

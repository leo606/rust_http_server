use crate::http::{Request, Response, StatusCode};
use std::io::Read;
use std::convert::TryFrom;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self) {
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
                                    dbg!(req);
                                    Response::new(StatusCode::Ok, Some("IT WORKS!".to_string() ))
                                },
                                Err(e) => {
                                    println!("failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
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

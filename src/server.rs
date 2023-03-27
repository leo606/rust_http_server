use crate::http::Request;
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
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

                            match Request::try_from(&buffer as &[u8]) {
                                Ok(req) => {
                                    dbg!(req);
                                },
                                Err(e) => println!("failed to parse a request: {}", e)
                            }

                            let res: &Result<Request, _> = &buffer[..].try_into();
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
